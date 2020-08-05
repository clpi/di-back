pub mod user;
pub mod record;
pub mod relations;
pub mod links;
pub mod entry;
pub mod item;
pub mod field;
pub mod group;
pub mod rule;
pub mod action;

use chrono::Utc;

pub use user::User;

pub enum Time {
    Now,
    Tomorrow,
    Yesterday,
}

impl Time {
    pub fn now() -> i32 { Utc::now().timestamp() as i32 }
}
