use rusoto_core::{Region, RusotoError};
use serde_dynamodb;
use std::collections::HashMap;
use std::env;

use rusoto_dynamodb::{
    BatchWriteItemError, BatchWriteItemInput, DynamoDb, DynamoDbClient, PutRequest, WriteRequest,
};

use models::Household;

pub struct HouseholdService;

impl HouseholdService {
    pub fn put(household: Household) -> Result<Household, RusotoError<BatchWriteItemError>> {
        let client = DynamoDbClient::new(Region::UsEast1);
        let put_requests: Vec<WriteRequest> = household
            .rsvps
            .iter()
            .map(|rsvp| WriteRequest {
                put_request: Some(PutRequest {
                    item: serde_dynamodb::to_hashmap(&rsvp).unwrap(),
                }),
                ..WriteRequest::default()
            })
            .collect();

        let mut request_items: HashMap<String, Vec<WriteRequest>> = HashMap::new();
        request_items.insert(env::var("RSVP_TABLE_NAME").unwrap(), put_requests);

        let batch_write_request_input = BatchWriteItemInput {
            request_items: request_items,
            ..BatchWriteItemInput::default()
        };

        match client.batch_write_item(batch_write_request_input).sync() {
            Ok(_result) => Ok(household),
            Err(error) => Err(error),
        }
    }
}
