mod acolyte;
mod acrobat;
mod assassin;
mod barbarian;
mod bard;
mod beast_master;
mod cleric;
mod druid;
mod fighter;
mod illusionist;
mod kineticist;
mod knight;
mod mage;
mod magic_user;
mod paladin;
mod ranger;
mod thief;

use acolyte::Acolyte;
use acrobat::Acrobat;
use assassin::Assassin;
use barbarian::Barbarian;
use bard::Bard;
use beast_master::BeastMaster;
use cleric::Cleric;
use druid::Druid;
use fighter::Fighter;
use illusionist::Illusionist;
use kineticist::Kineticist;
use knight::Knight;
use mage::Mage;
use magic_user::MagicUser;
use paladin::Paladin;
use ranger::Ranger;
use thief::Thief;

use crate::types::{AvailableClass, Class};

fn get_classes() -> Vec<Class> {
    vec![
        Acolyte::describe(),
        Acrobat::describe(),
        Assassin::describe(),
        Barbarian::describe(),
        Bard::describe(),
        BeastMaster::describe(),
        Cleric::describe(),
        Druid::describe(),
        Fighter::describe(),
        Illusionist::describe(),
        Kineticist::describe(),
        Knight::describe(),
        Mage::describe(),
        MagicUser::describe(),
        Paladin::describe(),
        Ranger::describe(),
        Thief::describe(),
    ]
}

pub fn available_classes() -> Vec<Class> {
    get_classes()
}
