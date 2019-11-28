use rusoto_core::Region;
use rusoto_dynamodb::{
    DynamoDbClient,
    PutItemError,
    WriteRequest,
    PutRequest
};
use serde_dynamodb;

use crate::{models::{Household}};

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
        
        // let put_requests = household.rsvps
        //     .into_iter()
        //     .map(|rsvp| WriteRequest {
        //         put_request: Some(PutRequest {
        //             item: serde_dynamodb::to_hashmap(&rsvp).unwrap()
        //         }),
        //         ..WriteRequest::default()
        //     })
        //     .collect();

        Ok(household)
    }
}