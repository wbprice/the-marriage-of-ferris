use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum DietaryRestrictions {
    Vegetarian,
    Vegan,
    Pescatarian,
    GlutenFree,
    DairyFree,
}
