use lambda_http::{lambda, IntoResponse, Request};
use lambda_runtime::{error::HandlerError, Context};
use serde_json::{json};
use std::ops::Deref;

use models::{
    CreateHouseholdRequestBody,
    Household
};
use services::{
    HouseholdService
};

fn main() {
    lambda!(handler)
}

fn handler(
    request: Request,
    _: Context
) -> Result<impl IntoResponse, HandlerError> {

    // Confirm that the body has the shape we expect it to have.
    let json = request.body().deref();
    let payload : Result<CreateHouseholdRequestBody, serde_json::Error> = serde_json::from_slice(json);

    // Handle success and error cases
    match payload {
        Ok(payload) => {
            let household_service = HouseholdService::new();
            let household = Household::new(Some(payload.people));

            match household_service.put(household) {
                Ok(response) => {
                    Ok(json!(serde_json::to_string(&response)?))
                },
                Err(_put_item_error) => {
                    Ok(json!("{\"message\": \"you are not good at life\"}"))
                }
            }
        },
        Err(err) => {
            dbg!(err);
            dbg!("something bad happened");
            Ok(json!("{\"message\": \"you are not good at life. base case\"}"))
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use lambda_http::{Body};

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