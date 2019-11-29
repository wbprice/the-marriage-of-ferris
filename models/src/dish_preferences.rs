use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum DishPreferences {
    Chicken,
    Steak,
    Pancakes,
}
