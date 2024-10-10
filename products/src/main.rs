use axum::{
    extract::{Json, State},
    response::IntoResponse,
    routing::get,
    Router,
};
use hyper::Server;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::Arc;
use std::{collections::BTreeSet, net::SocketAddr};
use std::{env, fs};

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
    top_secret_facts: Vec<TopSecretFact>,
}

#[tokio::main]
async fn main() {
    let employees: Vec<Employee> = load_file("src/employees.json");
    let products: Vec<Product> = load_file("src/products.json");
    let top_secret_facts: Vec<TopSecretFact> = load_file("src/top_secret_facts.json");

    let shared_state = Arc::new(AppState {
        employees: employees,
        products: products,
        top_secret_facts: top_secret_facts,
    });

    let app = Router::new()
        .route("/products", get(get_products))
        .route("/top_secret_facts", get(get_top_secret_facts))
        .route("/fact_types", get(get_fact_types))
        .route("/products/employees", get(get_products_employees))
        .with_state(shared_state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 8083));
    println!("Listening on {}", addr);
    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn get_products_employees(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    Json(state.employees.clone()).into_response()
}

// async fn find_employee_by_id(
//     Path(id): Path<u32>,
//     State(state): State<Arc<AppState>>,
// ) -> impl IntoResponse {
//     let employee = state.employees.iter().find(|e| e.id == id);
//     match employee {
//         Some(emp) => Json(emp.clone()).into_response(),
//         None => (axum::http::StatusCode::NOT_FOUND, "Employee not found").into_response(),
//     }
// }

async fn get_products(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    Json(state.products.clone()).into_response()
}

async fn get_top_secret_facts(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let facts = state.top_secret_facts.clone();
    Json(facts).into_response()
}

async fn get_fact_types(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let fact_types = state
        .top_secret_facts
        .iter()
        .map(|fact| match fact {
            TopSecretFact::DirectiveFact { fact_type, .. } => fact_type,
            TopSecretFact::EntityFact { fact_type, .. } => fact_type,
            TopSecretFact::MiscellaneousFact { fact_type, .. } => fact_type,
        })
        .collect::<BTreeSet<_>>();
    Json(fact_types).into_response()
}

fn load_file<T: for<'a> Deserialize<'a>>(file_path: &str) -> Vec<T> {
    // Get the current directory
    let current_dir = env::current_dir().expect("Unable to determine current directory");
    println!("Current directory: {:?}", current_dir);

    // Construct the path to the file
    let mut path = PathBuf::from(&current_dir);
    path.push(file_path);

    // Print the constructed path for debugging
    println!("Attempting to read file at path: {:?}", path);

    // Read the employee data at the start of the server
    let data = fs::read_to_string(&path).expect("Unable to read file");

    serde_json::from_str(&data).expect("JSON was not well-formatted")
}
