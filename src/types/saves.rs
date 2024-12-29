use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Deserialize, Serialize, Debug)]
pub struct Saves {
    #[serde(rename = "D")]
    pub doom: i64,

    #[serde(rename = "R")]
    pub ray: i64,

    #[serde(rename = "H")]
    pub hold: i64,

    #[serde(rename = "B")]
    pub blast: i64,

    #[serde(rename = "S")]
    pub spell: i64,
}

impl Display for Saves {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            " {} | {} | {} | {} | {} ",
            self.doom, self.ray, self.hold, self.blast, self.spell
        )?;

        Ok(())
    }
}
