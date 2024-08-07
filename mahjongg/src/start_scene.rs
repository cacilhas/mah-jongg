use godot::engine::{Control, IControl, InputEvent};
use godot::prelude::*;

#[derive(Debug, GodotClass)]
#[class(init, base=Control)]
pub(crate) struct StartScene {
	base: Base<Control>,
}

#[godot_api]
impl IControl for StartScene {
	fn unhandled_input(&mut self, event: Gd<InputEvent>) {
		if event.is_action_pressed("exit".into()) {
			if let Some(mut tree) = self.base_mut().get_tree() {
				tree.quit();
			} else {
				godot_error!("could not retrieve tree");
			}
		}
	}
}

#[godot_api]
impl StartScene {
	#[func]
	fn on_start_pressed(&mut self) {
		// TODO: change scene
	}
}
