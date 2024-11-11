use crate::types::{AvailableClass, Class};

#[derive(Default, Clone)]
pub struct Druid;

impl AvailableClass for Druid {
    fn describe() -> Class {
        serde_json::from_str(include_str!("definitions/druid.json"))
            .expect("JSON was not well formatted")
    }
}
