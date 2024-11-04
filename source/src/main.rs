use axum::{
    extract::Request,
    http::HeaderValue,
    middleware::Next,
    response::Response,
    routing::{get, post},
    Router,
};
use resource::resource_str;
use std::env;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        panic!("Invalid args: {:?}. Use big|medium|small", args)
    }

    let app = match args[1].as_str() {
        "big" => Router::new()
            .route("/big-json", get(big_data))
            .route("/graphql", post(big)),
        "medium" => Router::new()
            .route("/medium-json", get(medium_data))
            .route("/graphql", post(medium))
            .route("/big-json", get(medium_data)),
        "small" => Router::new()
            .route("/small-json", get(employees_data))
            .route("/employees", get(employees_data))
            .route("/graphql", post(employees)),
        _ => panic!("Invalid args: {:?}. Use big|medium|small", args),
    }
    .layer(axum::middleware::from_fn(http_cache_middleware));

    let addr = match args[1].as_str() {
        "big" => "127.0.0.1:4006",
        "medium" => "127.0.0.1:4006",
        "small" => "127.0.0.1:4001",
        _ => panic!("Invalid args: {:?}. Use big|medium|small", args),
    };

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn big() -> Response<String> {
    Response::new(resource_str!("big.json").to_string())
}

async fn big_data() -> Response<String> {
    Response::new(resource_str!("big-data.json").to_string())
}

async fn medium() -> Response<String> {
    Response::new(resource_str!("medium.json").to_string())
}

async fn medium_data() -> Response<String> {
    Response::new(resource_str!("medium-data.json").to_string())
}

async fn employees() -> Response<String> {
    Response::new(resource_str!("employees.json").to_string())
}

async fn employees_data() -> Response<String> {
    Response::new(resource_str!("employees-data.json").to_string())
}

async fn http_cache_middleware(request: Request, next: Next) -> Response {
    let mut response = next.run(request).await;

    response.headers_mut().insert(
        "Cache-control",
        HeaderValue::from_str("max-age=180, public").unwrap(),
    );

    response.headers_mut().insert(
        "Content-type",
        HeaderValue::from_str("application/json").unwrap(),
    );

    response
}
