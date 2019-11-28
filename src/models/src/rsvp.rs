use uuid::Uuid;

use crate::{
    models::{
        Person,
        ContactStatus,
        DietaryRestrictions,
        DishPreferences
    }
};

pub struct RSVP {
    pub household_id: Uuid,
    pub id: Uuid,
    pub name: String,
    pub email_address: String,
    pub attending: bool,
    pub contact_status: ContactStatus,
    pub dietary_restrictions: Option<DietaryRestrictions>,
    pub dietary_restrictions_other: Option<String>,
    pub dish_preferences: Option<DishPreferences>
}

impl RSVP {
    pub fn new(household_id: Uuid, person: Person) -> RSVP {
        RSVP {
            household_id,
            id: Uuid::new_v4(),
            name: person.name,
            email_address: person.email_address,
            attending: false,
            contact_status: ContactStatus::Uninvited,
            dietary_restrictions: None,
            dietary_restrictions_other: None,
            dish_preferences: None,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn rsvp_new_should_create_a_new_rsvp() {
        let uuid = Uuid::new_v4();
        let person = Person::new("Blaine Price".to_string(), "1wbprice@gmail.com".to_string());
        RSVP::new(uuid, person);
    }
}