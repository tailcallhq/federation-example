use axum::http::StatusCode;
use axum::{
    extract::{Path, Query, State},
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use hyper::Server;
use serde::{Deserialize, Serialize};
use std::env;
use std::path::PathBuf;
use std::{collections::HashMap, fs, net::SocketAddr, sync::Arc};

// Define your structs
#[derive(Serialize, Deserialize, Clone)]
struct Employee {
    id: u32,
    details: EmployeeDetails,
    role: EmployeeRole,
    notes: String,
    tag: String,
    start_date: String,
    updated_at: String,
}

#[derive(Serialize, Deserialize, Clone)]
struct EmployeeDetails {
    forename: String,
    surname: String,
    location: String,
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

#[derive(Serialize, Deserialize, Clone)]
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
    employees: Vec<Employee>,
    products: Vec<Product>,
}

// Define the handler to return an employee by ID
async fn get_employee_by_id(
    Path(id): Path<u32>,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    let employees = &state.employees;
    if let Some(employee) = employees.iter().find(|e| e.id == id) {
        (StatusCode::OK, Json(employee.clone())).into_response()
    } else {
        (
            StatusCode::NOT_FOUND,
            Json("Employee not found".to_string()),
        )
            .into_response()
    }
}

async fn get_department_employees(
    Path(department): Path<String>,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    let employees = &state.employees;
    let employees = employees
        .iter()
        .filter(|employee| employee.role.departments.contains(&department))
        .cloned()
        .collect::<Vec<_>>();
    (StatusCode::OK, Json(employees)).into_response()
}

async fn get_products(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    // Create a map to handle multiple occurrences of the same key

    let products: Vec<_> = state
        .products
        .iter()
        .map(|product| match product {
            Product::Consultancy(consultancy) => serde_json::to_value(consultancy.clone()).unwrap(),
            Product::Cosmo(cosmo) => serde_json::to_value(cosmo.clone()).unwrap(),
            Product::SDK(sdk) => serde_json::to_value(sdk.clone()).unwrap(),
        })
        .collect::<Vec<_>>();
    (StatusCode::OK, Json(products)).into_response()
}

async fn get_products_lead(
    State(state): State<Arc<AppState>>,
    Query(params): Query<Vec<(String, String)>>,
) -> impl IntoResponse {
    // Create a map to handle multiple occurrences of the same key
    let mut param_map: HashMap<String, Vec<String>> = HashMap::new();

    // Collect all query parameters into param_map
    for (key, value) in params {
        param_map.entry(key).or_default().push(value);
    }

    let products: Vec<_> = state
        .products
        .iter()
        .filter_map(|product| {
            let include = param_map.iter().all(|(key, values)| match key.as_str() {
                "type" => {
                    let product_type = match &product {
                        Product::Consultancy(_) => "consultancy",
                        Product::Cosmo(_) => "cosmo",
                        Product::SDK(_) => "sdk",
                    }
                    .to_string();
                    values.contains(&product_type)
                }
                "upc" => {
                    let product_upc = match &product {
                        Product::Consultancy(c) => &c.upc,
                        Product::Cosmo(c) => &c.upc,
                        Product::SDK(c) => &c.upc,
                    };
                    values.contains(product_upc)
                }
                _ => true,
            });
            if include {
                Some(match product {
                    Product::Consultancy(consultancy) => {
                        let mut hashmap: HashMap<String, serde_json::Value> = HashMap::new();
                        hashmap.insert(
                            "upc".to_string(),
                            serde_json::to_value(consultancy.upc.clone()).unwrap(),
                        );
                        hashmap.insert(
                            "lead".to_string(),
                            serde_json::to_value(consultancy.lead.clone()).unwrap(),
                        );
                        serde_json::to_value(hashmap).unwrap()
                    }
                    Product::Cosmo(cosmo) => {
                        let mut hashmap: HashMap<String, serde_json::Value> = HashMap::new();
                        hashmap.insert(
                            "upc".to_string(),
                            serde_json::to_value(cosmo.upc.clone()).unwrap(),
                        );
                        hashmap.insert(
                            "lead".to_string(),
                            serde_json::to_value(cosmo.lead.clone()).unwrap(),
                        );
                        serde_json::to_value(hashmap).unwrap()
                    }
                    Product::SDK(sdk) => {
                        let mut hashmap: HashMap<String, serde_json::Value> = HashMap::new();
                        hashmap.insert(
                            "upc".to_string(),
                            serde_json::to_value(sdk.upc.clone()).unwrap(),
                        );
                        hashmap.insert(
                            "lead".to_string(),
                            serde_json::to_value(sdk.owner.clone()).unwrap(),
                        );
                        serde_json::to_value(hashmap).unwrap()
                    }
                })
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    (StatusCode::OK, Json(products.first().unwrap())).into_response()
}

async fn get_products_engineers(
    State(state): State<Arc<AppState>>,
    Query(params): Query<Vec<(String, String)>>,
) -> impl IntoResponse {
    // Create a map to handle multiple occurrences of the same key
    let mut param_map: HashMap<String, Vec<String>> = HashMap::new();

    // Collect all query parameters into param_map
    for (key, value) in params {
        param_map.entry(key).or_default().push(value);
    }

    let products: Vec<_> = state
        .products
        .iter()
        .filter_map(|product| {
            let include = param_map.iter().all(|(key, values)| match key.as_str() {
                "type" => {
                    let product_type = match &product {
                        Product::Consultancy(_) => "consultancy",
                        Product::Cosmo(_) => "cosmo",
                        Product::SDK(_) => "sdk",
                    }
                    .to_string();
                    values.contains(&product_type)
                }
                "upc" => {
                    let product_upc = match &product {
                        Product::Consultancy(c) => &c.upc,
                        Product::Cosmo(c) => &c.upc,
                        Product::SDK(c) => &c.upc,
                    };
                    values.contains(product_upc)
                }
                _ => true,
            });
            if include {
                Some(match product {
                    Product::Consultancy(consultancy) => {
                        let mut hashmap: HashMap<String, serde_json::Value> = HashMap::new();
                        hashmap.insert(
                            "upc".to_string(),
                            serde_json::to_value(consultancy.upc.clone()).unwrap(),
                        );
                        hashmap.insert(
                            "engineers".to_string(),
                            serde_json::to_value(Vec::<String>::new()).unwrap(),
                        );
                        serde_json::to_value(hashmap).unwrap()
                    }
                    Product::Cosmo(cosmo) => {
                        let mut hashmap: HashMap<String, serde_json::Value> = HashMap::new();
                        hashmap.insert(
                            "upc".to_string(),
                            serde_json::to_value(cosmo.upc.clone()).unwrap(),
                        );
                        hashmap.insert(
                            "engineers".to_string(),
                            serde_json::to_value(cosmo.engineers.clone()).unwrap(),
                        );
                        serde_json::to_value(hashmap).unwrap()
                    }
                    Product::SDK(sdk) => {
                        let mut hashmap: HashMap<String, serde_json::Value> = HashMap::new();
                        hashmap.insert(
                            "upc".to_string(),
                            serde_json::to_value(sdk.upc.clone()).unwrap(),
                        );
                        hashmap.insert(
                            "engineers".to_string(),
                            serde_json::to_value(sdk.engineers.clone()).unwrap(),
                        );
                        serde_json::to_value(hashmap).unwrap()
                    }
                })
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    (StatusCode::OK, Json(products.first().unwrap())).into_response()
}

// Define the handler to filter employees based on query parameters, including multiple IDs
async fn filter_employees(
    Query(params): Query<Vec<(String, String)>>,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
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
                "forename" => values
                    .iter()
                    .any(|v| v.to_lowercase() == employee.details.forename.to_lowercase()),
                "surname" => values
                    .iter()
                    .any(|v| v.to_lowercase() == employee.details.surname.to_lowercase()),
                "department" => values.iter().any(|v| {
                    employee
                        .role
                        .departments
                        .iter()
                        .any(|d| d.to_lowercase() == v.to_lowercase())
                }),
                "engineer_type" => values.iter().any(|v| {
                    employee
                        .role
                        .engineer_type
                        .as_deref()
                        .map_or(false, |e| e.to_lowercase() == v.to_lowercase())
                }),
                "operator_type" => employee.role.operator_type.as_ref().map_or(false, |types| {
                    values
                        .iter()
                        .any(|v| types.iter().any(|t| t.to_lowercase() == v.to_lowercase()))
                }),
                "title" => values.iter().any(|v| {
                    employee
                        .role
                        .title
                        .iter()
                        .any(|t| t.to_lowercase() == v.to_lowercase())
                }),
                _ => true,
            })
        })
        .cloned()
        .collect();

    (StatusCode::OK, Json(filtered_employees))
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
        employees: employees,
        products: products,
    });

    // Build the router with the state and new routes
    let app = Router::new()
        .route("/employees/:id", get(get_employee_by_id))
        .route("/employees", get(filter_employees))
        .route("/employees/products", get(get_products))
        .route("/employees/products/lead", get(get_products_lead))
        .route("/employees/products/engineers", get(get_products_engineers))
        .route(
            "/employees/department/:department",
            get(get_department_employees),
        )
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
