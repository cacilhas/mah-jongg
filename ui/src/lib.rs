#[macro_use] mod macros;
mod scenes;

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
