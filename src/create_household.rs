use lambda_http::{lambda, IntoResponse, Request, Body};
use lambda_runtime::{error::HandlerError, Context};
use serde_json::json;

mod models;
use crate::models::{
    CreateHouseholdRequestBody
};

fn main() {
    lambda!(handler)
}

fn handler(
    request: Request,
    _: Context
) -> Result<impl IntoResponse, HandlerError> {

    if let Body::Text(text) = request.body() {
        dbg!(text);
        let parsed : CreateHouseholdRequestBody = serde_json::from_str(text)?;

    }

    Ok(json!("{\"message\": \"you are good at life\"}"))
}

#[cfg(test)]
mod test {
    use super::*;
    use lambda_http::{Body};

    #[test]
    fn test_should_handle_create_household_request() {
        let payload = r#"{
            people: [
                {
                    "name": "Blaine Price",
                    "emailAddress": "1wbprice@gmail.com"
                }
            ]
        }"#;

        let request = Request::new(Body::from(payload));
        handler(request, Context::default()).expect("Expected Ok() value");
    }
}