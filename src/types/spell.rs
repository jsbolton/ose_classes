use std::fmt::Display;

use serde::Deserialize;

#[derive(Default, Clone, Deserialize, Debug)]
pub struct SpellSlot {
    pub level: i64,

    pub slots: i64,
}

impl Display for SpellSlot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.level, self.slots)?;
        Ok(())
    }
}
