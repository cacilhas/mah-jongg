use godot::prelude::*;
use super::card::Card;

#[derive(Debug, GodotClass)]
#[class(init, base=Node2D)]
struct CardsContainer {
	base: Base<Node2D>,
	card1: Option<Gd<Card>>,
	card2: Option<Gd<Card>>,
	card3: Option<Gd<Card>>,
	card4: Option<Gd<Card>>,
	card5: Option<Gd<Card>>,
}

impl CardsContainer {
	unwrap![card1: Card];
	unwrap![card2: Card];
	unwrap![card3: Card];
	unwrap![card4: Card];
	unwrap![card5: Card];
}

#[godot_api]
impl INode2D for CardsContainer {
	fn ready(&mut self) {
		self.card1 = Some(self.base().get_node_as("Card1"));
		self.card2 = Some(self.base().get_node_as("Card2"));
		self.card3 = Some(self.base().get_node_as("Card3"));
		self.card4 = Some(self.base().get_node_as("Card4"));
		self.card5 = Some(self.base().get_node_as("Card5"));
	}
}

#[godot_api()]
impl CardsContainer {
	#[func]
	fn on_timer_timeout(&mut self) {
		self.card1().call("shuffle_facet".into(), &[]);
		self.card2().call("shuffle_facet".into(), &[]);
		self.card3().call("shuffle_facet".into(), &[]);
		self.card4().call("shuffle_facet".into(), &[]);
		self.card5().call("shuffle_facet".into(), &[]);
	}
}
