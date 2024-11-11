use crate::types::{AvailableClass, Class};

#[derive(Default, Clone)]
pub struct Knight;

impl AvailableClass for Knight {
    fn describe() -> Class {
        serde_json::from_str(include_str!("definitions/knight.json"))
            .expect("JSON was not well formatted")
    }
}
