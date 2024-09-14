use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct User {
    pub(crate) id: u32,
    pub(crate) name: String,
    pub(crate) email: String,
}