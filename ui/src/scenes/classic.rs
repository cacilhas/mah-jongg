use godot::engine::InputEvent;
use godot::prelude::*;

#[derive(Debug, GodotClass)]
#[class(init, base=Node2D)]
pub(crate) struct Classic {
	base: Base<Node2D>,
}

#[godot_api]
impl INode2D for Classic {
	fn unhandled_input(&mut self, event: Gd<InputEvent>) {
		if event.is_action_pressed("exit".into()) {
			self.base_mut()
				.get_tree()
				.expect("could not retrieve tree")
				.change_scene_to_file("res://start_scene.tscn".into());
		}
	}
}
