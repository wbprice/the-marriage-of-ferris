use lambda_http::{lambda, IntoResponse, Request, Response, Body};
use lambda_runtime::{error::HandlerError, Context};
use rusoto_core::RusotoError;
use serde_json::json;
use std::ops::Deref;

use models::{CreateHouseholdRequestBody, Household};
use services::HouseholdService;

fn main() {
    lambda!(handler)
}

async fn handler(request: Request, _: Context) -> Result<impl IntoResponse, HandlerError> {
    // Confirm that the body has the shape we expect it to have.
    let json = request.body().deref();
    let payload: Result<CreateHouseholdRequestBody, serde_json::Error> =
        serde_json::from_slice(json);

    // Handle success and error cases
    match payload {
        Ok(payload) => {
            let household = Household::new(Some(payload.people));

            match HouseholdService::put(household).await {
                // Success case
                Ok(response) => Ok(Response::builder()
                    .header("Access-Control-Allow-Origin", "*")
                    .status(200)
                    .body(serde_json::to_string(&response)?)),
                // Error cases
                Err(rusoto_error) => match rusoto_error {
                    // Handle Rusoto client errors
                    RusotoError::Service(service_error) => match service_error {
                        _ => Ok(Response::builder()
                            .header("Access-Control-Allow-Origin", "*")
                            .status(500)
                            .body(json!({"message": "An service error occurred"}).to_string()))
                    },
                    // Any other case
                    _ => Ok(Response::builder()
                        .header("Acesss-Control-Allow-Origin", "*")
                        .status(500)
                        .body("{}".to_string()))
                },
            }
        }
        // Handle client errors
        Err(serde_error) => match serde_error {
            _ => Ok(Response::builder()
                .header("Access-Control-Allow-Origin", "*")
                .status(400)
                .body("{}".to_string()))
        },
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use lambda_http::Body;

    #[test]
    fn test_create_household_should_handle() {
        let payload = r#"{
            "people": [
                {
                    "name": "Blaine Price",
                    "email_address": "1wbprice@gmail.com"
                }
            ]
        }"#;

        let request = Request::new(Body::from(payload));
        handler(request, Context::default()).expect("Expected Ok() value");
    }

    #[test]
    fn test_should_not_accept_bad_json() {
        let payload = r#"{
            people: [
                {
                    "name": "Blaine Price",
                    "email_address": "1wbprice@gmail.com"
                }
            ]
        }"#;

        let request = Request::new(Body::from(payload));
        handler(request, Context::default()).expect("Expected Ok() value");
    }
}
