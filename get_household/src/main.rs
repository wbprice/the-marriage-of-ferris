use lambda_http::{lambda, IntoResponse, Request, Response};
use lambda_runtime::{error::HandlerError, Context};

use services::HouseholdService;

fn main() {
    lambda!(handler)
}

fn handler(request: Request, _: Context) -> Result<impl IntoResponse, HandlerError> {
    let path_parameters = request.path_parameters();
    let uuid : Uuid = Uuid::parse_str(
        path_parameters.get("id").unwrap()
    ).unwrap();

    match HouseholdService::get(uuid) {
        Ok(household_response) => {
            match household_response {
                Some(record) => {
                    Ok(Response::builder()
                        .header("Access-Control-Allow-Origin", "*")
                        .status(200)
                        .body(serde_json::to_string(&record)?)
                        .unwrap()
                        .body()
                        .to_string())
                },
                None => {
                    Ok(Response::builder()
                        .header("Access-Control-Allow-Origin", "*")
                        .status(200)
                        .body(serde_json::to_string(&record)?)
                        .unwrap()
                        .body()
                        .to_string())
                }
            }
        },
        Err(error) => {

        }
    }
}