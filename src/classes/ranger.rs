use crate::types::{AvailableClass, Class};

#[derive(Default, Clone)]
pub struct Ranger;

impl AvailableClass for Ranger {
    fn describe() -> Class {
        serde_json::from_str(include_str!("definitions/ranger.json"))
            .expect("JSON was not well formatted")
    }
}
