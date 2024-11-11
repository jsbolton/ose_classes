use crate::types::{AvailableClass, Class};

#[derive(Default, Clone)]
pub struct Acolyte;

impl AvailableClass for Acolyte {
    fn describe() -> Class {
        serde_json::from_str(include_str!("definitions/acolyte.json"))
            .expect("JSON was not well formatted")
    }
}
