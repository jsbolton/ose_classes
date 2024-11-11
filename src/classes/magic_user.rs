use crate::types::{AvailableClass, Class};

#[derive(Default, Clone)]
pub struct MagicUser;

impl AvailableClass for MagicUser {
    fn describe() -> Class {
        serde_json::from_str(include_str!("definitions/magic_user.json"))
            .expect("JSON was not well formatted")
    }
}
