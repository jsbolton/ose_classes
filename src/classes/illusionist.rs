use crate::types::{AvailableClass, Class};

#[derive(Default, Clone)]
pub struct Illusionist;

impl AvailableClass for Illusionist {
    fn describe() -> Class {
        serde_json::from_str(include_str!("definitions/illusionist.json"))
            .expect("JSON was not well formatted")
    }
}
