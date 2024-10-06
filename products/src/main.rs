use axum::{
    extract::{Json, Path, State},
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use hyper::Server;
use serde::{Deserialize, Serialize};
use std::env;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::Mutex as TokioMutex;

#[derive(Debug, Deserialize, Serialize, Clone)]
struct Employee {
    id: u32,
    products: Vec<String>,
    notes: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(tag = "type")]
enum Product {
    Cosmo {
        upc: String,
        name: String,
        repository_url: String,
    },
    Consultancy {
        upc: String,
        name: String,
    },
    Documentation {
        url: String,
        urls: Vec<String>,
    },
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(tag = "type")]
enum TopSecretFact {
    DirectiveFact {
        title: String,
        description: String,
        fact_type: String,
    },
    EntityFact {
        title: String,
        description: String,
        fact_type: String,
    },
    MiscellaneousFact {
        title: String,
        description: String,
        fact_type: String,
    },
}

struct AppState {
    employees: Vec<Employee>,
    products: Vec<Product>,
    top_secret_facts: TokioMutex<Vec<TopSecretFact>>,
}

#[tokio::main]
async fn main() {
    let current_dir = env::current_dir().expect("Unable to determine current directory");
    println!("Current directory: {:?}", current_dir);

    // Construct the path to the file
    let mut employee_path = PathBuf::from(&current_dir);
    let mut product_path = PathBuf::from(&current_dir);
    let mut facts_path = PathBuf::from(&current_dir);
    employee_path.push("src/employees.json");
    product_path.push("src/products.json");
    facts_path.push("src/top_secret_facts.json");
    // Load data from JSON files
    let employees: Vec<Employee> =
        serde_json::from_str(&std::fs::read_to_string(&employee_path).unwrap()).unwrap();
    let products: Vec<Product> =
        serde_json::from_str(&std::fs::read_to_string(&product_path).unwrap()).unwrap();
    let top_secret_facts: Vec<TopSecretFact> =
        serde_json::from_str(&std::fs::read_to_string(&facts_path).unwrap()).unwrap();

    let shared_state = Arc::new(AppState {
        employees,
        products,
        top_secret_facts: TokioMutex::new(top_secret_facts),
    });

    let app = Router::new()
        .route("/employees/:id", get(find_employee_by_id))
        .route("/products", get(get_products))
        .route("/top_secret_facts", get(get_top_secret_facts))
        .with_state(shared_state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 8083));
    println!("Listening on {}", addr);
    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn find_employee_by_id(
    Path(id): Path<u32>,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    let employee = state.employees.iter().find(|e| e.id == id);
    match employee {
        Some(emp) => Json(emp.clone()).into_response(),
        None => (axum::http::StatusCode::NOT_FOUND, "Employee not found").into_response(),
    }
}

async fn get_products(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    Json(state.products.clone()).into_response()
}

async fn get_top_secret_facts(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let facts = state.top_secret_facts.lock().await;
    Json(facts.clone()).into_response()
}

#[derive(Debug, Deserialize)]
struct AddFactInput {
    title: String,
    description: String,
    fact_type: String,
}
