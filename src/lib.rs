use classes::get_classes;
use types::Class;

mod classes;
mod types;

pub fn available_classes() -> Vec<Class> {
    get_classes()
}
