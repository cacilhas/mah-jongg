use godot::engine::InputEvent;
use godot::global::randi_range;
use godot::prelude::*;

use super::card::Card;

#[derive(Debug, GodotClass)]
#[class(init, base=Node2D)]
pub(crate) struct Classic {
	base: Base<Node2D>,
}

#[godot_api]
impl INode2D for Classic {
    fn ready(&mut self) {
        let cards_wrapper: Gd<Node2D> = self.base().get_node_as("Cards");
        let mut counter = take_counter();

        for card in cards_wrapper.get_children().iter_shared() {
            let mut card: Gd<Card> = card.try_cast().expect("expected card, got node");
            let idx = randi_range(0, counter.len() as i64 - 1) as usize;
			let current = &mut counter[idx];
			let jdx = Variant::from(current.idx as i64);
			if current.incr_is_last() {
				counter.remove(idx);
			}
            card.call("set_facet_index".into(), &[jdx]);
        }
    }

	fn unhandled_input(&mut self, event: Gd<InputEvent>) {
		if event.is_action_pressed("exit".into()) {
			self.base_mut()
				.get_tree()
				.expect("could not retrieve tree")
				.change_scene_to_file("res://start_scene.tscn".into());
		}
	}
}

fn take_counter() -> Vec<CardType> {
    let mut counter: Vec<CardType> = (0..42).map(CardType::new).collect();
    for _i in 0..6 {
        let idx = randi_range(0, counter.len() as i64) as usize;
		counter.remove(idx);
    }
	counter
}

struct CardType {
	idx: usize,
	amount: u8,
}

impl CardType {
	fn new(idx: usize) -> Self {
		Self { idx, amount: 0 }
	}

	fn incr_is_last(&mut self) -> bool {
		if self.amount < 4 {
			self.amount += 1;
		}
		self.amount == 4
	}
}
