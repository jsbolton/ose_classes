use crate::types::{AvailableClass, Class};

#[derive(Default, Clone)]
pub struct Mage;

impl AvailableClass for Mage {
    fn describe() -> Class {
        serde_json::from_str(include_str!("definitions/mage.json"))
            .expect("JSON was not well formatted")
    }
}
