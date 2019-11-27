use uuid::Uuid;

use crate::models::{
    Person,
    RSVP
};

pub struct Household {
    pub id: Uuid,
    pub rsvps: Vec<RSVP>
}

impl Household {
    fn new(people: Option<Vec<Person>>) -> Household {
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

