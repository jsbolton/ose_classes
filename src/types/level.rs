use serde::Deserialize;

use super::{HitDie, Saves, Spells};

#[derive(Default, Clone, Debug, Deserialize)]
pub struct Level {
    pub level: i64,

    #[serde(rename = "xp")]
    pub experience_points: i64,

    #[serde(rename = "mod")]
    pub to_hit_modifier: i64,

    pub saves: Saves,

    #[serde(rename = "hd")]
    pub hit_die: HitDie,

    pub spells: Option<Spells>,
}
