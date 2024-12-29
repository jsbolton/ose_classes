use serde::{Deserialize, Serialize};

use super::{HitDie, Saves, Spells};

#[derive(Default, Clone, Debug, Deserialize, Serialize)]
pub struct Level {
    pub level: i64,

    #[serde(rename = "xp")]
    pub experience_points: i64,

    #[serde(rename = "mod")]
    pub to_hit_modifier: i64,

    pub saves: Saves,

    #[serde(rename = "hd")]
    pub hit_die: HitDie,

    /// Only used by casters
    pub spells: Option<Spells>,

    /// Only used by Kineticist class
    pub ac: Option<i64>,

    /// Only used by Kineticist class
    pub powers: Option<i64>,

    /// Only used by skill-based classes (Acolyte, Acrobat, Assassin, Mage, Thief)
    pub skills: Option<i64>,
}
