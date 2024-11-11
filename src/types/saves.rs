use serde::Deserialize;
use strum_macros::Display;

#[derive(Clone, Display, Debug, Deserialize)]
pub enum Saves {
    /// Alt: Death or Poison
    #[strum(to_string = "D")]
    #[serde(rename = "D")]
    Doom,

    /// Alt: Wands
    #[strum(to_string = "R")]
    #[serde(rename = "R")]
    Ray,

    /// Alt: Paralysis or Petrification
    #[strum(to_string = "H")]
    #[serde(rename = "H")]
    Hold,

    /// Alt: Breath attacks
    #[strum(to_string = "B")]
    #[serde(rename = "B")]
    Blast,

    /// Alt: Spells, Rods, or Staves
    #[strum(to_string = "S")]
    #[serde(rename = "S")]
    Spell,
}

#[derive(Clone, Debug, Deserialize)]
pub struct SaveTarget {
    /// Type of Save to roll against
    pub save: Saves,

    /// Target value to match or beat to succeed save
    pub target: i64,
}

impl ToString for SaveTarget {
    fn to_string(&self) -> String {
        format!("{} | {}", self.save, self.target)
    }
}
