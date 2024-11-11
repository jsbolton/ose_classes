use crate::types::{AvailableClass, Class};

#[derive(Default, Clone)]
pub struct Paladin;

impl AvailableClass for Paladin {
    fn describe() -> Class {
        serde_json::from_str(include_str!("definitions/paladin.json"))
            .expect("JSON was not well formatted")
    }
}
