use strum::{Display, EnumString};

use crate::types::Class;

#[derive(EnumString, Display)]
#[strum(ascii_case_insensitive)]
pub enum CharacterClass {
    Acolyte,
    Acrobat,
    Assassin,
    Barbarian,
    Bard,
    BeastMaster,
    Cleric,
    Druid,
    Fighter,
    Illusionist,
    Kineticist,
    Knight,
    Mage,
    MagicUser,
    Paladin,
    Ranger,
    Thief,
}

impl CharacterClass {
    pub fn describe(&self) -> Class {
        match self {
            Self::Acolyte => load_class_def(include_str!("definitions/acolyte.json")),
            Self::Acrobat => load_class_def(include_str!("definitions/acrobat.json")),
            Self::Assassin => load_class_def(include_str!("definitions/assassin.json")),
            Self::Barbarian => load_class_def(include_str!("definitions/barbarian.json")),
            Self::Bard => load_class_def(include_str!("definitions/bard.json")),
            Self::BeastMaster => load_class_def(include_str!("definitions/beast_master.json")),
            Self::Cleric => load_class_def(include_str!("definitions/cleric.json")),
            Self::Druid => load_class_def(include_str!("definitions/druid.json")),
            Self::Fighter => load_class_def(include_str!("definitions/fighter.json")),
            Self::Illusionist => load_class_def(include_str!("definitions/illusionist.json")),
            Self::Kineticist => load_class_def(include_str!("definitions/kineticist.json")),
            Self::Knight => load_class_def(include_str!("definitions/knight.json")),
            Self::Mage => load_class_def(include_str!("definitions/mage.json")),
            Self::MagicUser => load_class_def(include_str!("definitions/magic_user.json")),
            Self::Paladin => load_class_def(include_str!("definitions/paladin.json")),
            Self::Ranger => load_class_def(include_str!("definitions/ranger.json")),
            Self::Thief => load_class_def(include_str!("definitions/thief.json")),
        }
    }
}

fn load_class_def(class_def: &str) -> Class {
    serde_json::from_str(class_def).expect("JSON was not well formatted")
}
