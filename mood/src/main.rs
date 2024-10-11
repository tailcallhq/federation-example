use axum::http::StatusCode;
use axum::{extract::State, response::IntoResponse, routing::post, Json, Router};
use hyper::Server;
use serde::{Deserialize, Serialize};
use std::env;
use std::path::PathBuf;
use std::sync::Mutex;
use std::{fs, net::SocketAddr, sync::Arc};

#[derive(Debug, Clone, Deserialize, Serialize)]
enum Mood {
    HAPPY,
    SAD,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct Employee {
    id: u32,
    current_mood: Mood,
}

#[derive(Serialize, Deserialize, Debug)]
struct Input {
    #[serde(rename = "employeeID")]
    employee_id: u32,
    mood: Mood,
}

async fn update_mood(
    State(state): State<Arc<AppState>>,
    Json(input): Json<Input>,
) -> impl IntoResponse {
    let mut db = state.mood_db.lock().unwrap();

    match db
        .iter_mut()
        .find(|employee| employee.id == input.employee_id)
    {
        Some(employee) => {
            employee.current_mood = input.mood;
            Json(employee).into_response()
        }
        None => {
            let error_response = Json(format!("Employee with id {} not found", input.employee_id));
            (StatusCode::NOT_FOUND, error_response).into_response()
        }
    }
}

struct AppState {
    mood_db: Mutex<Vec<Employee>>,
}

#[tokio::main]
async fn main() {
    // Get the current directory
    let current_dir = env::current_dir().expect("Unable to determine current directory");
    println!("Current directory: {:?}", current_dir);

    // Construct the path to the file
    let mut path = PathBuf::from(&current_dir);
    path.push("src/mood.json");

    // Print the constructed path for debugging
    println!("Attempting to read file at path: {:?}", path);

    // Read the employee data at the start of the server
    let data = fs::read_to_string(&path).expect("Unable to read file");
    let employees_mood: Vec<Employee> =
        serde_json::from_str(&data).expect("JSON was not well-formatted");

    // Create shared application state
    let shared_state = Arc::new(AppState {
        mood_db: Mutex::new(employees_mood),
    });

    // Build the router with the state and new routes
    let app = Router::new()
        .route("/mood", post(update_mood))
        .with_state(shared_state);

    // Define the address to serve the application
    let addr = SocketAddr::from(([127, 0, 0, 1], 8071));
    println!("Listening on http://{}", addr);

    // Run the server using axum's Server
    Server::bind(&addr)
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .unwrap();
}
