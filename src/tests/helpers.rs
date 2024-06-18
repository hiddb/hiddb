use crate::api::types::*;
use actix_http;
use actix_web::dev::Service;

use actix_web::{error, test, Error};
use derive_more::{Display, Error};

#[derive(Debug, Display, Error)]
#[display(fmt = "Response Error: {}", cause)]
struct ResponseError {
    cause: String,
}

impl error::ResponseError for ResponseError {}

pub async fn create_collection<T: Service<Request = actix_http::Request, Response = actix_web::dev::ServiceResponse, Error = actix_web::Error>>(
    app: &mut T,
    collection_name: &str,
) -> CollectionResponse {
    // Create collection
    let req = test::TestRequest::post()
        .uri("/collection")
        .set_json(&CollectionRequest {
            collection_name: collection_name.to_owned(),
        })
        .to_request();
    let resp = app.call(req).await.unwrap();
    let response_body = match resp.response().body().as_ref() {
        Some(actix_web::body::Body::Bytes(bytes)) => bytes,
        _ => panic!("Response error"),
    };

    let collection_response: CollectionResponse = serde_json::from_str(std::str::from_utf8(&response_body).unwrap()).unwrap();
    return collection_response;
}

pub async fn get_collections<T: Service<Request = actix_http::Request, Response = actix_web::dev::ServiceResponse, Error = actix_web::Error>>(
    app: &mut T,
) -> CollectionsResponse {
    // Create collection
    let req = test::TestRequest::get().uri("/collection").to_request();
    let resp = app.call(req).await.unwrap();
    let response_body = match resp.response().body().as_ref() {
        Some(actix_web::body::Body::Bytes(bytes)) => bytes,
        _ => panic!("Response error"),
    };

    let collection_response: CollectionsResponse = serde_json::from_str(std::str::from_utf8(&response_body).unwrap()).unwrap();
    return collection_response;
}
