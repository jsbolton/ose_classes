use serde::Deserialize;
use strum_macros::Display;

#[derive(Debug, Eq, PartialEq, Display, Clone, Deserialize)]
pub enum Ability {
    #[strum(to_string = "STR")]
    Strength,

    #[strum(to_string = "INT")]
    Intelligence,

    #[strum(to_string = "WIS")]
    Wisdom,

    #[strum(to_string = "DEX")]
    Dexterity,

    #[strum(to_string = "CON")]
    Constitution,

    #[strum(to_string = "CHA")]
    Charisma,
}

#[derive(Clone, Debug, Deserialize)]
pub struct AbilityRequirement {
    pub ability: Ability,

    pub requirement: i64,
}
