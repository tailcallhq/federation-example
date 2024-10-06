use axum::{
    extract::{Path, Query, State},
    routing::get,
    response::IntoResponse,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs, net::SocketAddr, sync::Arc};
use axum::http::StatusCode;
use hyper::Server;
use std::path::PathBuf;
use std::env;

// Define your structs
#[derive(Serialize, Deserialize, Clone)]
struct Employee {
    id: u32,
    details: EmployeeDetails,
    role: EmployeeRole,
    notes: String,
    start_date: String,
    updated_at: String,
}

#[derive(Serialize, Deserialize, Clone)]
struct EmployeeDetails {
    forename: String,
    surname: String,
    location: Location,
    past_locations: Vec<PastLocation>,
}

#[derive(Serialize, Deserialize, Clone)]
struct Location {
    key: LocationKey,
}

#[derive(Serialize, Deserialize, Clone)]
struct LocationKey {
    name: String,
}

#[derive(Serialize, Deserialize, Clone)]
struct PastLocation {
    #[serde(rename = "type")]
    loc_type: String,
    name: String,
    country: Country,
}

#[derive(Serialize, Deserialize, Clone)]
struct Country {
    key: LocationKey,
}

#[derive(Serialize, Deserialize, Clone)]
struct EmployeeRole {
    departments: Vec<String>,
    #[serde(rename = "engineer_type")]
    engineer_type: Option<String>,
    #[serde(rename = "operator_type")]
    operator_type: Option<Vec<String>>,
    title: Vec<String>,
}

// Define the shared application state
struct AppState {
    employees: Arc<Vec<Employee>>,
}


// Define the handler to return an employee by ID
async fn get_employee_by_id(Path(id): Path<u32>, State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let employees = &state.employees;
    if let Some(employee) = employees.iter().find(|e| e.id == id) {
        (StatusCode::OK, Json(employee.clone())).into_response()
    } else {
        (StatusCode::NOT_FOUND, Json("Employee not found".to_string())).into_response()
    }
}

// Define the handler to filter employees based on query parameters, including multiple IDs
async fn filter_employees(Query(params): Query<Vec<(String, String)>>, State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let employees = &state.employees;

    // Create a map to handle multiple occurrences of the same key
    let mut param_map: HashMap<String, Vec<String>> = HashMap::new();

    // Collect all query parameters into param_map
    for (key, value) in params {
        param_map.entry(key).or_default().push(value);
    }

    // Filter employees based on the provided query parameters
    let filtered_employees: Vec<Employee> = employees
        .iter()
        .filter(|employee| {
            param_map.iter().all(|(key, values)| match key.as_str() {
                "id" => values.contains(&employee.id.to_string()),
                "forename" => values.iter().any(|v| v.to_lowercase() == employee.details.forename.to_lowercase()),
                "surname" => values.iter().any(|v| v.to_lowercase() == employee.details.surname.to_lowercase()),
                "department" => values.iter().any(|v| employee.role.departments.iter().any(|d| d.to_lowercase() == v.to_lowercase())),
                "engineer_type" => values.iter().any(|v| employee.role.engineer_type.as_deref().map_or(false, |e| e.to_lowercase() == v.to_lowercase())),
                "operator_type" => employee.role.operator_type.as_ref().map_or(false, |types| values.iter().any(|v| types.iter().any(|t| t.to_lowercase() == v.to_lowercase()))),
                "title" => values.iter().any(|v| employee.role.title.iter().any(|t| t.to_lowercase() == v.to_lowercase())),
                _ => true,
            })
        })
        .cloned()
        .collect();

    (StatusCode::OK, Json(filtered_employees))
}

#[tokio::main]
async fn main() {
    // Get the current directory
    let current_dir = env::current_dir().expect("Unable to determine current directory");
    println!("Current directory: {:?}", current_dir);

    // Construct the path to the file
    let mut path = PathBuf::from(&current_dir);
    path.push("src/employee-data.json");

    // Print the constructed path for debugging
    println!("Attempting to read file at path: {:?}", path);

    // Read the employee data at the start of the server
    let data = fs::read_to_string(&path).expect("Unable to read file");
    let employees: Vec<Employee> = serde_json::from_str(&data).expect("JSON was not well-formatted");

    // Create shared application state
    let shared_state = Arc::new(AppState {
        employees: Arc::new(employees),
    });

    // Build the router with the state and new routes
    let app = Router::new()
        .route("/employees/:id", get(get_employee_by_id))
        .route("/employees", get(filter_employees))
        .with_state(shared_state);

    // Define the address to serve the application
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("Listening on http://{}", addr);

    // Run the server using axum's Server
    Server::bind(&addr)
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .unwrap();
}
