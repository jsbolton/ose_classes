use crate::types::{AvailableClass, Class};

#[derive(Default, Clone)]
pub struct Cleric;

impl AvailableClass for Cleric {
    fn describe() -> Class {
        serde_json::from_str(include_str!("definitions/cleric.json"))
            .expect("JSON was not well formatted")
    }
}
