use crate::types::{AvailableClass, Class};

#[derive(Default, Clone)]
pub struct Acrobat;

impl AvailableClass for Acrobat {
    fn describe() -> Class {
        serde_json::from_str(include_str!("definitions/acrobat.json"))
            .expect("JSON was not well formatted")
    }
}
