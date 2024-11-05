mod model;

use axum::{extract::Query, response::IntoResponse, routing::get, Json, Router};
use hyper::Server;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct BigResponseParams {
    artificial_delay: Option<u64>,
    big_objects: Option<usize>,
    nested_objects: Option<usize>,
    deeply_nested_objects: Option<usize>,
}

async fn big_response_handler(Query(params): Query<BigResponseParams>) -> impl IntoResponse {
    let artificial_delay = params.artificial_delay.unwrap_or(0);
    let big_objects = params.big_objects.unwrap_or(10);
    let nested_objects = params.nested_objects.unwrap_or(5);
    let deeply_nested_objects = params.deeply_nested_objects.unwrap_or(3);

    if artificial_delay > 0 {
        tokio::time::sleep(std::time::Duration::from_millis(artificial_delay)).await;
    }

    let mut big = Vec::with_capacity(big_objects);
    for _ in 0..big_objects {
        let mut nested = Vec::with_capacity(nested_objects);
        for _ in 0..nested_objects {
            let mut deeply_nested = Vec::with_capacity(deeply_nested_objects);
            for _ in 0..deeply_nested_objects {
                deeply_nested.push(model::DeeplyNestedObject::default());
            }
            nested.push(model::NestedObject {
                deeply_nested_objects: deeply_nested,
            });
        }
        big.push(model::BigObject {
            nested_objects: nested,
        });
    }

    Json(big)
}

#[tokio::main]
async fn main() {
    // Build the router with the state and new routes
    let app = Router::new().route("/big-json", get(big_response_handler));

    // Define the address to serve the application
    let addr = SocketAddr::from(([0, 0, 0, 0], 8088));
    println!("Listening on http://{}", addr);

    // Run the server using axum's Server
    Server::bind(&addr)
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .unwrap();
}
