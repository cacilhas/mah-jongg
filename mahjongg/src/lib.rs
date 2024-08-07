use godot::global::randomize;
use godot::prelude::*;

struct MahJonggPlugin;

#[gdextension]
unsafe impl ExtensionLibrary for MahJonggPlugin {
    fn on_level_init(level: InitLevel) {
        if level == InitLevel::Core {
            randomize();
        }
    }
}