use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Default, Deserialize, Serialize, Clone, Debug)]
pub struct HitDie {
    #[serde(rename = "type")]
    pub die_type: i64,

    pub count: i64,

    pub modifier: Option<i64>,

    pub con_mod_applies: bool,
}

impl Display for HitDie {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let modifier = self
            .modifier
            .map(|m| format!("+{}", m))
            .unwrap_or_else(|| "".into());

        let con_mod: String = self
            .con_mod_applies
            .then(|| "".into())
            .unwrap_or_else(|| "*".into());

        write!(f, "{}d{}{}{}", self.count, self.die_type, modifier, con_mod)?;

        Ok(())
    }
}
