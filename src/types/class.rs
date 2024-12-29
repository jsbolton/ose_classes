use std::fmt::Display;

use comfy_table::{Cell, CellAlignment, Table};
use num_format::{Locale, ToFormattedString};
use serde::{Deserialize, Serialize};
use strum::Display;

use super::{AbilityRequirement, Level};

#[derive(Default, Clone, Debug, Deserialize, Serialize)]
pub struct Class {
    pub title: String,

    pub levels: Vec<Level>,

    pub source: Source,

    pub requirements: Vec<AbilityRequirement>,
}

impl Class {
    fn has_spells(&self) -> bool {
        self.levels.iter().any(|l| l.spells.is_some())
    }

    fn has_ac(&self) -> bool {
        self.levels.iter().any(|l| l.ac.is_some())
    }

    fn has_powers(&self) -> bool {
        self.levels.iter().any(|l| l.powers.is_some())
    }

    fn has_skills(&self) -> bool {
        self.levels.iter().any(|l| l.skills.is_some())
    }
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

        if self.has_spells() {
            headers.push("Spells (Level:Slots)");
        }

        if self.has_ac() {
            headers.push("AC");
        }

        if self.has_powers() {
            headers.push("Powers");
        }

        if self.has_skills() {
            headers.push("Skills");
        }

        table.set_header(headers);

        self.levels.iter().for_each(|level| {
            let xp = level.experience_points.to_formatted_string(&Locale::en);

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
                Cell::new(level.saves.clone()).set_alignment(CellAlignment::Center),
            ];

            if let Some(spells) = level.spells.as_ref() {
                columns.push(Cell::new(spells).set_alignment(CellAlignment::Center))
            }

            if let Some(ac) = level.ac.as_ref() {
                columns.push(Cell::new(ac).set_alignment(CellAlignment::Center))
            }

            if let Some(powers) = level.powers.as_ref() {
                columns.push(Cell::new(powers).set_alignment(CellAlignment::Center))
            }

            if let Some(skills) = level.skills.as_ref() {
                columns.push(Cell::new(skills).set_alignment(CellAlignment::Center))
            }

            table.add_row(columns);
        });

        writeln!(f, "{table}")?;

        Ok(())
    }
}

#[derive(Default, Clone, Debug, Eq, PartialEq, Display, Deserialize, Serialize)]
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
