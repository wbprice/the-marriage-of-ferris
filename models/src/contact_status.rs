use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum ContactStatus {
    Uninvited,
    Invited,
    Reminded,
}
