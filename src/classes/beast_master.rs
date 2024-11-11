use crate::types::{AvailableClass, Class};

#[derive(Default, Clone)]
pub struct BeastMaster;

impl AvailableClass for BeastMaster {
    fn describe() -> Class {
        serde_json::from_str(include_str!("definitions/beast_master.json"))
            .expect("JSON was not well formatted")
    }
}
