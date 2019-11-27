use lambda_runtime::{error::HandlerError, lambda, Context};
use serde_json::Value;

mod models;
use crate::models::{
    Person,
    Household
};

fn main() {
    lambda!(handler)
}

fn handler(
    event: Value,
    _: Context
) -> Result<Value, HandlerError> {
    Ok(event)
}