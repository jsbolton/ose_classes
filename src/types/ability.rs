use serde::Deserialize;
use strum_macros::Display;

#[derive(Debug, Eq, PartialEq, Display, Clone, Deserialize)]
pub enum Ability {
    #[strum(to_string = "STR")]
    #[serde(rename = "STR")]
    Strength,

    #[strum(to_string = "INT")]
    #[serde(rename = "INT")]
    Intelligence,

    #[strum(to_string = "WIS")]
    #[serde(rename = "WIS")]
    Wisdom,

    #[strum(to_string = "DEX")]
    #[serde(rename = "DEX")]
    Dexterity,

    #[strum(to_string = "CON")]
    #[serde(rename = "CON")]
    Constitution,

    #[strum(to_string = "CHA")]
    #[serde(rename = "CHA")]
    Charisma,
}

#[derive(Clone, Debug, Deserialize)]
pub struct AbilityRequirement {
    pub ability: Ability,

    pub requirement: i64,
}
