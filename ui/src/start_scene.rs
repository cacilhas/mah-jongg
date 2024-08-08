use godot::engine::{Control, IControl, InputEvent};
use godot::prelude::*;

#[derive(Debug, GodotClass)]
#[class(init, base=Control)]
pub(crate) struct StartScene {
	base: Base<Control>,
	#[export]
	next_scene: Gd<PackedScene>,
}

#[godot_api]
impl IControl for StartScene {
	fn unhandled_input(&mut self, event: Gd<InputEvent>) {
		if event.is_action_pressed("exit".into()) {
			self.base_mut()
				.get_tree()
				.expect("could not retrieve tree")
				.quit();
		}
	}
}

#[godot_api]
impl StartScene {
	#[func]
	fn on_start_pressed(&mut self) {
		let next_scene = self.next_scene.clone();
		self.base_mut()
			.get_tree()
			.expect("could not retrieve tree")
			.change_scene_to_packed(next_scene);
	}
}
