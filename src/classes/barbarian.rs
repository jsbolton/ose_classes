use crate::types::{AvailableClass, Class};

#[derive(Default, Clone)]
pub struct Barbarian;

impl AvailableClass for Barbarian {
    fn describe() -> Class {
        serde_json::from_str(include_str!("definitions/barbarian.json"))
            .expect("JSON was not well formatted")
    }
}
