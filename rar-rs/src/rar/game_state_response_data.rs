use std::any::Any;

use crate::rar::game_state::GameStateResponseData;

#[derive(Debug)]
pub struct GameStateResponseDataSelectWorld {
	world: String,
}

impl GameStateResponseDataSelectWorld {
	pub fn new(world: &str) -> Self {
		Self {
			world: world.to_string(),
		}
	}

	pub fn world(&self) -> &str {
		&self.world
	}
}

impl GameStateResponseData for GameStateResponseDataSelectWorld {
	fn name(&self) -> &str {
		"SelectWorld"
	}

	fn as_any(&self) -> &(dyn Any) {
		self
	}
}
