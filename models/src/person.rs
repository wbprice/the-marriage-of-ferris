use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Person {
    pub email_address: String,
    pub name: String,
}

impl Person {
    pub fn new(name: String, email_address: String) -> Person {
        Person {
            name,
            email_address,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn person_new_should_create_a_new_person() {
        let person = Person::new("Blaine Price".to_string(), "1wbprice@gmail.com".to_string());
        assert_eq!(person.email_address, "1wbprice@gmail.com");
        assert_eq!(person.name, "Blaine Price");
    }
}
