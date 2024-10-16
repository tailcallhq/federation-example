mod model;

use axum::http::StatusCode;
use axum::{
    async_trait,
    extract::{FromRequestParts, Query},
    http::request::Parts,
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use hyper::Server;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, net::SocketAddr};

pub struct Qs2<T>(pub T);

#[async_trait]
impl<S, T> FromRequestParts<S> for Qs2<T>
where
    S: Send + Sync,
    T: serde::de::DeserializeOwned,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(parts: &mut Parts, _: &S) -> Result<Self, Self::Rejection> {
        let query = parts.uri.query().unwrap();
        Ok(Self(serde_qs::from_str(query).unwrap()))
    }
}

#[derive(Debug, Deserialize, Serialize)]
enum EnumType {
    A,
    B,
    C,
}

#[derive(Deserialize, Debug, Serialize)]
struct InputArg {
    #[serde(rename = "enum")]
    enum_type: Option<EnumType>,
    #[serde(rename = "string")]
    string: Option<String>,
    #[serde(rename = "strings")]
    vec_string: Option<Vec<String>>,
    #[serde(rename = "enums")]
    vec_enum_type: Option<Vec<EnumType>>,
}
#[derive(Deserialize, Debug, Serialize)]
struct RootQuery {
    #[serde(flatten)]
    input: InputArg,
}

async fn handle_big_abstract_response() -> impl IntoResponse {
    Json(model::ABigObject::default()).into_response()
}

async fn root_field_with_input(Qs2(params): Qs2<RootQuery>) -> impl IntoResponse {
    Json(params).into_response()
}

async fn header_value(
    Query(params): Query<HashMap<String, String>>,
    headers: axum::http::HeaderMap,
) -> impl IntoResponse {
    let header_name = params.get("name").cloned().unwrap_or_default();
    headers
        .get(&header_name)
        .and_then(|value| value.to_str().ok())
        .map(String::from)
        .map(Json)
        .unwrap_or_else(|| Json("Header not found".to_string()))
}

#[derive(Deserialize, Debug)]
struct DelayParams {
    ms: u64,
    response: String,
}

async fn handle_delay(Query(params): Query<DelayParams>) -> impl IntoResponse {
    tokio::time::sleep(std::time::Duration::from_millis(params.ms)).await;
    Json(params.response).into_response()
}

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
    let app = Router::new()
        .route("/header-value", get(header_value))
        .route("/root-field-with-input", get(root_field_with_input))
        .route("/delay", get(handle_delay))
        .route("/big-abstract-response", get(handle_big_abstract_response))
        .route("/big-response", get(big_response_handler));

    // Define the address to serve the application
    let addr = SocketAddr::from(([127, 0, 0, 1], 8088));
    println!("Listening on http://{}", addr);

    // Run the server using axum's Server
    Server::bind(&addr)
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .unwrap();
}
