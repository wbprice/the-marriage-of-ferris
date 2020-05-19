use lambda_http::{lambda, IntoResponse, Request, RequestExt, Response};
use lambda_runtime::{error::HandlerError, Context};
use uuid::Uuid;

use services::HouseholdService;

fn main() {
    lambda!(handler)
}

fn handler(request: Request, _: Context) -> Result<impl IntoResponse, HandlerError> {
    let path_parameters = request.path_parameters();
    let id = path_parameters.get("id");

    if let Some(uuid) = Uuid::parse_str(id) {
        match HouseholdService::get(uuid) {
            Ok(household_response) => match household_response {
                Some(record) => {
                    Ok(Response::builder()
                        .header("Access-Control-Allow-Origin", "*")
                        .status(200)
                        .body(serde_json::to_string(&record)?));
                }
                None => {
                    Ok(Response::builder()
                        .header("Access-Control-Allow-Origin", "*")
                        .status(404)
                        .body());
                }
            },

            Err(error) => {
                Ok(Response::builder()
                    .header("Access-Control-Allow-Origin", "*")
                    .status(500)
                    .body(error.to_string()));
            }
        }
    } else {
        Ok(Response::builder()
            .header("Access-Control-Allow-Origin", "*")
            .status(400)
            .body());
    }
}
