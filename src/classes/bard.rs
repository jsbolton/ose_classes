use crate::types::{AvailableClass, Class};

#[derive(Default, Clone)]
pub struct Bard;

impl AvailableClass for Bard {
    fn describe() -> Class {
        serde_json::from_str(include_str!("definitions/bard.json"))
            .expect("JSON was not well formatted")
    }
}
