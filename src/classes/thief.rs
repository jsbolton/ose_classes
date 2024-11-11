use crate::types::{AvailableClass, Class};

#[derive(Default, Clone)]
pub struct Thief;

impl AvailableClass for Thief {
    fn describe() -> Class {
        serde_json::from_str(include_str!("definitions/thief.json"))
            .expect("JSON was not well formatted")
    }
}
