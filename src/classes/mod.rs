mod barbarian;
mod cleric;
mod druid;
mod fighter;
mod magic_user;
mod thief;

use barbarian::Barbarian;
use cleric::Cleric;
use druid::Druid;
use fighter::Fighter;
use magic_user::MagicUser;
use thief::Thief;

use crate::types::{AvailableClass, Class};

pub fn get_classes() -> Vec<Class> {
    vec![
        Barbarian::describe(),
        Cleric::describe(),
        Druid::describe(),
        Fighter::describe(),
        MagicUser::describe(),
        Thief::describe(),
    ]
}
