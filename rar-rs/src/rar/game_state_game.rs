use oml_game::renderer::Renderer;
use oml_game::window::WindowUpdateContext;
use oml_game::system::System;

use crate::rar::entities::{EntityConfigurationManager, EntityId, Player};
use crate::rar::entities::entity::Entity;
use crate::rar::GameState;
use crate::rar::EntityUpdateContext;

pub struct GameStateGame {
	entity_configuration_manager: EntityConfigurationManager,
	player: Player,	
}

impl GameStateGame {
	pub fn new() -> Self {
		Self {
			player: Player::new(),
			entity_configuration_manager: EntityConfigurationManager::new(),			
		}
	}
}

impl GameState for GameStateGame {
	fn setup(&mut self, system: &mut System) {
		self.entity_configuration_manager
			.load(system, "todo_filename");

		self.player.setup(
			self.entity_configuration_manager
				.get_config(EntityId::PLAYER as u32),
		);
		self.player.respawn();		
	}
	fn teardown(&mut self) {
		self.player.teardown();		
	}
	fn update(&mut self, wuc: &mut WindowUpdateContext) {
		let mut euc = EntityUpdateContext::new();

		let player_direction = if wuc.is_key_pressed('a' as u8) {
			-1
		} else if wuc.is_key_pressed('d' as u8) {
			1
		} else {
			0
		};
		euc = euc
			.set_time_step(wuc.time_step)
			.set_player_direction(player_direction);

		self.player.update(&mut euc);		
	}
	fn render(&mut self, renderer: &mut Renderer) {
		self.player.render(renderer);		
	}
	fn name(&self) -> &str {
		"[GameState] Game"
	}
}
