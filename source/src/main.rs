use axum::{
    extract::Request,
    http::HeaderValue,
    middleware::Next,
    response::Response,
    routing::{get, post},
    Router,
};
use resource::resource_str;
use std::{env, net::SocketAddr};
use tokio::signal;
use axum_server::Handle;

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


    let addr: SocketAddr = "127.0.0.1:4006".parse().unwrap();

    let handle = Handle::new();

    let server = axum_server::bind(addr)
        .handle(handle.clone())
        .serve(app.into_make_service());

    println!("Listening on {}", addr);

    tokio::select! {
        _ = server => {
            eprintln!("Server exited unexpectedly");
        }
        _ = shutdown_signal() => {
            println!("Shutdown signal received, initiating graceful shutdown...");
            handle.graceful_shutdown(Some(std::time::Duration::from_secs(30)));
        }
    }
}

async fn shutdown_signal() {
    signal::ctrl_c()
        .await
        .expect("Failed to install CTRL+C signal handler");
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
        "Cache-Control",
        HeaderValue::from_static("max-age=180, public"),
    );

    response.headers_mut().insert(
        "Content-Type",
        HeaderValue::from_static("application/json"),
    );

    response
}
