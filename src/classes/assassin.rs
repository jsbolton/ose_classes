use crate::types::{AvailableClass, Class};

#[derive(Default, Clone)]
pub struct Assassin;

impl AvailableClass for Assassin {
    fn describe() -> Class {
        serde_json::from_str(include_str!("definitions/assassin.json"))
            .expect("JSON was not well formatted")
    }
}
