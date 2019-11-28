use rusoto_core::Region;
use std::collections::{HashMap};
use std::env;
use serde_dynamodb;

use rusoto_dynamodb::{
    DynamoDbClient,
    PutItemError,
    WriteRequest,
    BatchWriteItemInput,
    PutRequest
};

use models::{Household};

pub struct HouseholdService {
    client: DynamoDbClient
}

impl HouseholdService {
    pub fn new() -> HouseholdService {
        HouseholdService {
            client: DynamoDbClient::new(Region::UsEast1)
        }
    }

    pub fn put(&self, household: Household) -> Result<Household, PutItemError> {

        let put_requests = household.rsvps
            .into_iter()
            .map(|rsvp| {
                let hashmap = 

                WriteRequest {
                    put_request: Some(PutRequest {
                        item: serde_dynamodb::to_hashmap(&rsvp).unwrap()
                    }),
                    ..WriteRequest::default()
                }
            )
            .collect();

            let mut request_items : HashMap<String, Vec<WriteRequest>> = HashMap::new();
            
            request_items.insert(env::var("RSVP_TABLE_NAME").unwrap(), put_requests);
    
            let batch_write_request_input = BatchWriteItemInput {
                request_items: request_items,
                ..BatchWriteItemInput::default()
            };
    
            match self.client.batch_write_item(batch_write_request_input).sync() {
                Ok(_result) => {
                    Ok(rsvps)
                },
                Err(error) => {
                    Err(error)
                }
            }

        Ok(household)
    }
}