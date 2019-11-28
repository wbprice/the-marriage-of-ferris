use uuid::Uuid;
use serde_derive::{Serialize, Deserialize};

use crate::{
    Person,
    RSVP
};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateHouseholdRequestBody {
    pub people: Vec<Person>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Household {
    pub id: Uuid,
    pub rsvps: Vec<RSVP>
}

impl Household {
    pub fn new(people: Option<Vec<Person>>) -> Household {
        let id = Uuid::new_v4();

        match people {
            Some(people) => {
                let rsvps : Vec<RSVP> = people
                    .into_iter()
                    .map(|person| RSVP::new(id, person))
                    .collect();

                Household {
                    id,
                    rsvps
                }
            }
            None => {
                Household {
                    id, 
                    rsvps: vec!()
                }
            } 
        }
    }
}

