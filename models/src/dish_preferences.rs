use serde_derive::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum DishPreferences {
    Chicken,
    Steak,
    Pancakes
}