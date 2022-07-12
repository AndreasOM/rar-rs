use oml_game::renderer::Renderer;
use oml_game::window::WindowUpdateContext;
use oml_game::system::System;

use crate::rar::entities::{Background, EntityConfigurationManager, EntityId, EntityManager, Player};
use crate::rar::entities::entity::Entity;
use crate::rar::GameState;
use crate::rar::EntityUpdateContext;

pub struct GameStateGame {
	entity_configuration_manager: EntityConfigurationManager,
	entity_manager: EntityManager,
}

impl GameStateGame {
	pub fn new() -> Self {
		Self {
			entity_manager: EntityManager::new(),
			entity_configuration_manager: EntityConfigurationManager::new(),
		}
	}
}

impl GameState for GameStateGame {
	fn setup(&mut self, system: &mut System) {
		self.entity_configuration_manager
			.load(system, "todo_filename");

		self.entity_manager.setup();

		// add player
		let mut player = Player::new();
		player.setup(
			self.entity_configuration_manager
				.get_config(EntityId::PLAYER as u32),
		);
		player.respawn();
		self.entity_manager.add( Box::new( player ) );

		// add background
		let mut background = Background::new();
		background.setup(
			self.entity_configuration_manager
				.get_config(EntityId::BACKGROUND as u32),
		);
		self.entity_manager.add( Box::new( background ) );
	}
	fn teardown(&mut self) {
		self.entity_manager.teardown();
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

		for e in self.entity_manager.iter_mut() {
			e.update( &mut euc );
		}
	}
	fn render(&mut self, renderer: &mut Renderer) {
		for e in self.entity_manager.iter_mut() {
			e.render( renderer );
		}
	}
	fn name(&self) -> &str {
		"[GameState] Game"
	}
}
