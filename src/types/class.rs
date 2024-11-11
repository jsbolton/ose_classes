use std::{fmt::Display, ops::Not};

use comfy_table::{Cell, CellAlignment, Table};
use num_format::{Locale, ToFormattedString};
use serde::Deserialize;
use strum_macros::Display;

use super::{AbilityRequirement, Level};

#[derive(Default, Clone, Debug, Deserialize)]
pub struct Class {
    pub title: String,

    pub levels: Vec<Level>,

    pub source: Source,

    pub requirements: Vec<AbilityRequirement>,
}

impl Display for Class {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.title)?;
        writeln!(f, "Source: {}", self.source)?;

        writeln!(f, "\nAbility Requirements:")?;

        if self.requirements.is_empty() {
            writeln!(f, "None")?;
        } else {
            for r in self.requirements.clone() {
                writeln!(f, "{} : {}", r.ability, r.requirement)?;
            }
        }

        writeln!(f, "\nLevels")?;

        let mut table = Table::new();
        let mut headers = vec![
            "Level",
            "XP",
            "HD",
            "Attack Mod",
            "Saves (D | R | H | B | S)",
        ];

        let _ = self.levels.first().map(|l| {
            if !l.spells.is_empty() {
                headers.push("Spells (Level:Slots)");
            }
        });

        table.set_header(headers);

        self.levels.iter().for_each(|level| {
            let xp = level.experience_points.to_formatted_string(&Locale::en);

            let saves = level
                .saves
                .clone()
                .into_iter()
                .map(|l| l.target.to_string())
                .collect::<Vec<String>>()
                .join(" | ");

            let attack_mod = level
                .to_hit_modifier
                .gt(&0)
                .then(|| format!("+{}", &level.to_hit_modifier))
                .unwrap_or_else(|| level.to_hit_modifier.to_string());

            let mut columns = vec![
                Cell::new(level.level.to_string()).set_alignment(CellAlignment::Center),
                Cell::new(xp).set_alignment(CellAlignment::Center),
                Cell::new(level.hit_die.clone()).set_alignment(CellAlignment::Center),
                Cell::new(attack_mod).set_alignment(CellAlignment::Center),
                Cell::new(saves).set_alignment(CellAlignment::Center),
            ];

            let spells = level
                .spells
                .clone()
                .into_iter()
                .map(|s| s.to_string())
                .collect::<Vec<String>>()
                .join(" | ");

            if !spells.is_empty() {
                columns.push(Cell::new(spells).set_alignment(CellAlignment::Center));
            }

            table.add_row(columns);
        });

        writeln!(f, "{table}")?;

        Ok(())
    }
}

pub trait AvailableClass {
    fn describe() -> Class;
}

#[derive(Default, Clone, Debug, Eq, PartialEq, Display, Deserialize)]
pub enum Source {
    #[default]
    #[strum(serialize = "C", to_string = "Classic Fantasy")]
    #[serde(rename = "C")]
    Classic,

    #[strum(serialize = "A", to_string = "Advanced Fantasy")]
    #[serde(rename = "A")]
    Advanced,

    #[strum(to_string = "Carcass Crawler 1")]
    CC1,

    #[strum(to_string = "Carcass Crawler 3")]
    CC3,

    #[strum(disabled, serialize = "D", to_string = "Dolmenwood Player's Book")]
    #[serde(rename = "D")]
    Dolmenwood,
}
