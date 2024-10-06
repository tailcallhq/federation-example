use axum::{
    extract::{Path, State},
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use hyper::Server;
use serde::{Deserialize, Serialize};
use std::env;
use std::path::PathBuf;
use std::{fs, net::SocketAddr, sync::Arc};

#[derive(Debug, Deserialize, Serialize)]
struct Employee {
    id: u32,
    hobbies: Vec<Hobby>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type")] // This will add a "type" field for hobby kind (e.g., "Exercise", "Gaming")
enum Hobby {
    Exercise { category: String },
    Gaming {
        genres: Vec<String>,
        name: String,
        years_of_experience: f64,
    },
    Flying {
        plane_models: Vec<String>,
        years_of_experience: f64,
    },
    Other { name: String },
    Programming { languages: Vec<String> },
    Travelling {
        countries_lived: Vec<Country>,
    },
}

#[derive(Debug, Deserialize, Serialize)]
struct Country {
    key: CountryKey,
}

#[derive(Debug, Deserialize, Serialize)]
struct CountryKey {
    name: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct Sdk {
    upc: String,
    client_languages: Vec<String>,
}

struct AppState {
    employees: Arc<Vec<Employee>>,
}

async fn find_employee_by_id(Path(id): Path<u32>, State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let employee = state.employees.iter().find(|e| e.id == id);
    match employee {
        Some(emp) => Json(emp).into_response(),
        None => (axum::http::StatusCode::NOT_FOUND, "Employee not found").into_response(),
    }
}

async fn find_sdk_by_upc(Path(upc): Path<String>) -> impl IntoResponse {
    // Simulate the SDK resolver as in the Go example
    if upc == "sdk" {
        let sdk = Sdk {
            upc,
            client_languages: vec!["Rust".to_string(), "Typescript".to_string()],
        };
        Json(sdk).into_response()
    } else {
        (axum::http::StatusCode::NOT_FOUND, "SDK not found").into_response()
    }
}

#[tokio::main]
async fn main() {
    // Get the current directory
    let current_dir = env::current_dir().expect("Unable to determine current directory");
    println!("Current directory: {:?}", current_dir);

    // Construct the path to the file
    let mut path = PathBuf::from(&current_dir);
    path.push("src/hobbies.json");

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
        .route("/employees/:id/hobbies", get(find_employee_by_id))
        .route("/sdk/:upc", get(find_sdk_by_upc))
        .with_state(shared_state);

    // Define the address to serve the application
    let addr = SocketAddr::from(([127, 0, 0, 1], 8082));
    println!("Listening on http://{}", addr);

    // Run the server using axum's Server
    Server::bind(&addr)
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .unwrap();
}
