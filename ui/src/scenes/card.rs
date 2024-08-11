use godot::engine::{Area2D, IArea2D, Sprite2D, Texture2D};
use godot::global::randi_range;
use godot::prelude::*;

#[derive(Debug, GodotClass)]
#[class(init, base=Area2D)]
pub(crate) struct Card {
	base: Base<Area2D>,
	sprite: Option<Gd<Sprite2D>>,
	facet_index: i64,
	#[export]
	facets: Array<Gd<Texture2D>>,
}

impl Card {
	unwrap![sprite: Sprite2D];
}

#[godot_api]
impl IArea2D for Card {
	fn ready(&mut self) {
		self.sprite = Some(self.base().get_node_as("Sprite"));
		self.update_facet();
	}
}

#[godot_api]
impl Card {
	#[func]
	fn update_facet(&mut self) {
		let facet = self.facets.at(self.facet_index as usize).clone();
		self.sprite().set_texture(facet);
	}

	#[func]
	fn shuffle_facet(&mut self) {
		self.facet_index = randi_range(0, self.len() - 1);
		self.update_facet();
	}

	#[func]
	fn len(&self) -> i64 {
		self.facets.len() as i64
	}

	#[func]
	fn set_facet_index(&mut self, facet_index: i64) {
		self.facet_index = facet_index;
		self.update_facet();
	}

	#[func]
	fn get_face_index(&self) -> i64 {
		self.facet_index
	}
}
