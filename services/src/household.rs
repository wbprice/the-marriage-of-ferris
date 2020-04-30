use rusoto_core::{Region, RusotoError};
use serde_dynamodb;
use std::collections::HashMap;
use std::env;
use uuid::Uuid;

use rusoto_dynamodb::{
    BatchWriteItemError, BatchWriteItemInput, DynamoDb, DynamoDbClient, PutRequest, QueryError,
    QueryInput, WriteRequest, AttributeValue
};

use models::{
    Household,
    RSVP
};

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

    pub fn get(household_id: Uuid) -> Result<Option<Household>, RusotoError<QueryError>> {
        let client = DynamoDbClient::new(Region::UsEast1);
        let mut query = HashMap::new();
        query.insert(
            String::from(":household_id"),
            AttributeValue {
                s: Some(household_id.to_string()),
                ..Default::default()
            },
        );

        let query_input = QueryInput {
            table_name: env::var("RSVP_TABLE_NAME").unwrap(),
            key_condition_expression: Some("household_id = :household_id".to_string()),
            expression_attribute_values: Some(query),
            ..QueryInput::default()
        };

        match client.query(query_input).sync() {
            Ok(response) => match response.items {
                Some(items) => {
                    let rsvps : Vec<RSVP> = items.into_iter()
                        .map(|item| serde_dynamodb::from_hashmap(item).unwrap())
                        .collect();
                    let household_id : Uuid = rsvps[0].household_id;

                    Ok(Some(Household {
                        id: household_id,
                        rsvps
                    }))
                }
                None => {
                    Ok(None))
                }
            },
            Err(error) => Err(error)
        }
    }
}
