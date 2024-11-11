use crate::types::{AvailableClass, Class};

#[derive(Default, Clone)]
pub struct Fighter;

impl AvailableClass for Fighter {
    fn describe() -> Class {
        serde_json::from_str(include_str!("definitions/fighter.json"))
            .expect("JSON was not well formatted")
    }
}
