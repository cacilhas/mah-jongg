#[macro_use] mod macros;

mod card;
mod cards_container;
mod start_scene;

use godot::global::randomize;
use godot::prelude::*;

struct MahJonggUIPlugin;

#[gdextension]
unsafe impl ExtensionLibrary for MahJonggUIPlugin {
    fn on_level_init(level: InitLevel) {
        if level == InitLevel::Core {
            randomize();
        }
    }
}
