use serde_derive::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum DietaryRestrictions {
    Vegetarian,
    Vegan,
    Pescatarian,
    GlutenFree,
    DairyFree
}