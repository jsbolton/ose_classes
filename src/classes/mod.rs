mod cleric;
mod fighter;
mod magic_user;
mod thief;

use cleric::Cleric;
use fighter::Fighter;
use magic_user::MagicUser;
use thief::Thief;

use crate::types::{AvailableClass, Class};

pub fn get_classes() -> Vec<Class> {
    vec![
        Cleric::describe(),
        Fighter::describe(),
        MagicUser::describe(),
        Thief::describe(),
    ]
}
