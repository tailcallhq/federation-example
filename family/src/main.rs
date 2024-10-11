use axum::{
    extract::{Query, State},
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use hyper::Server;
use serde::{Deserialize, Serialize};
use std::env;
use std::path::PathBuf;
use std::{fs, net::SocketAddr, sync::Arc};

#[derive(Debug, Deserialize)]
struct SearchInput {
    has_pets: Option<bool>,
    nationality: Option<String>,
    nested: Option<NestedSearchInput>,
}

#[derive(Debug, Deserialize)]
struct NestedSearchInput {
    marital_status: Option<String>,
    has_children: Option<bool>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct Employee {
    id: u32,
    details: Details,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct Details {
    forename: String,
    surname: String,
    middlename: Option<String>,
    has_children: Option<bool>,
    marital_status: Option<String>,
    nationality: String,
    pets: Option<Vec<Pet>>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct Pet {
    class: String,
    gender: String,
    name: String,
    #[serde(rename = "type")]
    pet_type: Option<String>,
}

async fn filter_employees(
    Query(params): Query<SearchInput>,
    Query(query): Query<Vec<(String, String)>>,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    let mut employees = (*state.employees).clone(); // Dereference Arc to get Vec<Employee>

    // Apply filters based on SearchInput
    if let Some(has_pets) = params.has_pets {
        employees.retain(|e| match &e.details.pets {
            Some(pets) if has_pets => !pets.is_empty(),
            None if !has_pets => true,
            _ => false,
        });
    }

    if let Some(ref nationality) = params.nationality {
        employees.retain(|e| e.details.nationality.eq_ignore_ascii_case(nationality));
    }

    if let Some(ref nested) = params.nested {
        if let Some(has_children) = nested.has_children {
            employees.retain(|e| e.details.has_children == Some(has_children));
        }

        if let Some(ref marital_status) = nested.marital_status {
            employees.retain(|e| match &e.details.marital_status {
                Some(ms) => ms.eq_ignore_ascii_case(marital_status),
                None => false,
            });
        }
    }

    let ids = query
        .into_iter()
        .filter_map(|(key, value)| if key == "id" { Some(value) } else { None })
        .collect::<Vec<_>>();

    if ids.len() > 0 {
        employees.retain(|e| ids.contains(&e.id.to_string()));
    }

    Json(
        employees
            .into_iter()
            .map(|employee| employee.id)
            .collect::<Vec<_>>(),
    ) // Return filtered employees
}

async fn get_employee_details_by_id(
    Query(ids): Query<Vec<(String, u32)>>,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    let employees = state.employees.clone();
    let filtered_employees: Vec<Details> = employees
        .iter()
        .filter(|e| ids.iter().any(|(key, id)| key == "id" && e.id == *id))
        .cloned()
        .map(|employee| employee.details)
        .collect();

    Json(filtered_employees)
}

// Define the shared application state
struct AppState {
    employees: Arc<Vec<Employee>>,
}

#[tokio::main]
async fn main() {
    // Get the current directory
    let current_dir = env::current_dir().expect("Unable to determine current directory");
    println!("Current directory: {:?}", current_dir);

    // Construct the path to the file
    let mut path = PathBuf::from(&current_dir);
    path.push("src/family.json");

    // Print the constructed path for debugging
    println!("Attempting to read file at path: {:?}", path);

    // Read the employee data at the start of the server
    let data = fs::read_to_string(&path).expect("Unable to read file");
    let employees: Vec<Employee> =
        serde_json::from_str(&data).expect("JSON was not well-formatted");

    // Create shared application state
    let shared_state = Arc::new(AppState {
        employees: Arc::new(employees),
    });

    // Build the router with the state and new routes
    let app = Router::new()
        .route("/family/employees", get(filter_employees))
        .route("/family/employee-details", get(get_employee_details_by_id)) // New route for batch employee details
        .with_state(shared_state);

    // Define the address to serve the application
    let addr = SocketAddr::from(([127, 0, 0, 1], 8081));
    println!("Listening on http://{}", addr);

    // Run the server using axum's Server
    Server::bind(&addr)
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .unwrap();
}
