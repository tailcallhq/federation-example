use axum::http::{HeaderValue, StatusCode};
use axum::middleware::Next;
use axum::response::Response;
use axum::routing::post;
use axum::{
    extract::{Path, State},
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use hyper::{Request, Server};
use serde::{Deserialize, Serialize};
use std::env;
use std::path::PathBuf;
use std::sync::RwLock;
use std::{fs, net::SocketAddr, sync::Arc};

// Define your structs
#[derive(Serialize, Deserialize, Clone)]
struct Employee {
    id: u32,
    details: EmployeeDetails,
    role: RoleType,
    notes: String,
    tag: String,
    #[serde(rename = "startDate")]
    start_date: String,
    #[serde(rename = "updatedAt")]
    updated_at: String,
}

#[derive(Serialize, Deserialize, Clone)]
struct EmployeeDetails {
    forename: String,
    surname: String,
    location: String,
}

#[derive(Serialize, Deserialize, Clone)]
enum RoleType {
    Engineer(EngineerRole),
    Marketer(MarketerRole),
    Operator(OperatorRole),
}

impl RoleType {
    fn departments_contains(&self, department_name: &String) -> bool {
        match self {
            RoleType::Engineer(engineer_role) => {
                engineer_role.departments.contains(department_name)
            }
            RoleType::Marketer(marketer_role) => {
                marketer_role.departments.contains(department_name)
            }
            RoleType::Operator(operator_role) => {
                operator_role.departments.contains(department_name)
            }
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
struct EngineerRole {
    departments: Vec<String>,
    #[serde(rename = "engineerType")]
    engineer_type: Option<String>,
    title: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone)]
struct MarketerRole {
    departments: Vec<String>,
    title: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone)]
struct OperatorRole {
    departments: Vec<String>,
    title: Vec<String>,
    #[serde(rename = "operatorType")]
    operator_type: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
enum Product {
    Consultancy(Consultancy),
    Cosmo(Cosmo),
    SDK(SDK),
}

#[derive(Serialize, Deserialize, Clone)]
struct Consultancy {
    upc: String,
    lead: Employee,
}

#[derive(Serialize, Deserialize, Clone)]
struct Cosmo {
    upc: String,
    lead: Employee,
    engineers: Vec<Employee>,
}

#[derive(Serialize, Deserialize, Clone)]
struct SDK {
    upc: String,
    owner: Employee,
    engineers: Vec<Employee>,
}

// Define the shared application state
struct AppState {
    employees: RwLock<Vec<Employee>>,
    products: Vec<Product>,
}

// Define the handler to return an employee by ID
async fn get_employee_by_id(
    Path(id): Path<u32>,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    let employees = state.employees.read().unwrap();
    if let Some(employee) = employees.iter().find(|e| e.id == id) {
        (StatusCode::OK, Json(employee.clone())).into_response()
    } else {
        (StatusCode::OK, Json(serde_json::Value::Null)).into_response()
    }
}

async fn get_department_employees(
    Path(department): Path<String>,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    let employees = state.employees.read().unwrap();
    let employees = employees
        .iter()
        .filter(|employee| employee.role.departments_contains(&department))
        .cloned()
        .collect::<Vec<_>>();
    (StatusCode::OK, Json(employees)).into_response()
}

async fn get_products(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    // Create a map to handle multiple occurrences of the same key

    let products: Vec<_> = state.products.clone();
    (StatusCode::OK, Json(products)).into_response()
}

async fn get_product_info(
    Path(upc): Path<String>,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    let product: Option<_> = state.products.iter().find_map(|product| match product {
        Product::Consultancy(consultancy) => {
            if consultancy.upc != upc {
                return None;
            }

            Some(serde_json::to_value(consultancy).unwrap())
        }
        Product::Cosmo(cosmo) => {
            if cosmo.upc != upc {
                return None;
            }

            Some(serde_json::to_value(cosmo).unwrap())
        }
        Product::SDK(sdk) => {
            if sdk.upc != upc {
                return None;
            }

            Some(serde_json::to_value(sdk).unwrap())
        }
    });

    match product {
        Some(product) => (StatusCode::OK, Json(product)).into_response(),
        None => StatusCode::NOT_FOUND.into_response(),
    }
}

// Updated handler to use POST
async fn update_employee_tag(
    Path(id): Path<u32>,
    State(state): State<Arc<AppState>>,
    Json(payload): Json<serde_json::Value>,
) -> impl IntoResponse {
    let mut employees = state.employees.write().unwrap();
    if let Some(employee) = employees.iter_mut().find(|e| e.id == id) {
        if let Some(new_tag) = payload.get("tag").and_then(|v| v.as_str()) {
            employee.tag = new_tag.to_string();
            return Json(employee).into_response();
        } else {
            return StatusCode::BAD_REQUEST.into_response();
        }
    } else {
        return StatusCode::NOT_FOUND.into_response();
    }
}

async fn filter_employees(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    (
        StatusCode::OK,
        Json(state.employees.read().unwrap().clone()),
    )
}

#[tokio::main]
async fn main() {
    let employees: Vec<Employee> = load_file("src/employee-data.json");

    let products = vec![
        Product::Consultancy(Consultancy {
            upc: "consultancy".to_string(),
            lead: employees.get(0).unwrap().clone(),
        }),
        Product::Cosmo(Cosmo {
            upc: "cosmo".to_string(),
            lead: employees.get(1).unwrap().clone(),
            engineers: vec![
                employees.get(2).unwrap().clone(),
                employees.get(3).unwrap().clone(),
                employees.get(4).unwrap().clone(),
            ],
        }),
        Product::SDK(SDK {
            upc: "sdk".to_string(),
            owner: employees.get(5).unwrap().clone(),
            engineers: vec![
                employees.get(6).unwrap().clone(),
                employees.get(7).unwrap().clone(),
            ],
        }),
    ];

    // Create shared application state
    let shared_state = Arc::new(AppState {
        employees: RwLock::new(employees),
        products: products,
    });

    // Build the router with the state and new routes
    let app = Router::new()
        .route("/employees", get(filter_employees))
        .route("/employees/:id", get(get_employee_by_id))
        .route("/employees/:id/tag", post(update_employee_tag))
        .route("/employees/products", get(get_products))
        .route("/employees/products/:upc", get(get_product_info))
        .route(
            "/employees/department/:department",
            get(get_department_employees),
        )
        .layer(axum::middleware::from_fn(http_cache_middleware))
        .with_state(shared_state);

    // Define the address to serve the application
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    println!("Listening on http://{}", addr);

    // Run the server using axum's Server
    Server::bind(&addr)
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .unwrap();
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

async fn http_cache_middleware<T>(request: Request<T>, next: Next<T>) -> Response {
    let mut response = next.run(request).await;

    response.headers_mut().insert(
        "Cache-control",
        HeaderValue::from_str("max-age=180, public").unwrap(),
    );

    response
}
