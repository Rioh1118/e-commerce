use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct User {
    pub(crate) id: u32,
    pub(crate) name: String,
    pub(crate) email: String,
    pub(crate) password_hash: String,
    pub(crate) created_at: DateTime<Local>,
    pub(crate) updated_at: DateTime<Local>,
    pub(crate) is_active: bool,
    pub(crate) last_login: Option<DateTime<Local>>,
    pub(crate) role: crate::utils::role::Role,
}
