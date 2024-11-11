use crate::types::{AvailableClass, Class};

#[derive(Default, Clone)]
pub struct Kineticist;

impl AvailableClass for Kineticist {
    fn describe() -> Class {
        serde_json::from_str(include_str!("definitions/kineticist.json"))
            .expect("JSON was not well formatted")
    }
}
