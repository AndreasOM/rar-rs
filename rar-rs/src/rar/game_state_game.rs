use oml_game::renderer::Renderer;
use oml_game::system::System;
use oml_game::window::WindowUpdateContext;

use crate::rar::entities::entity::Entity;
use crate::rar::entities::{
	Background, EntityConfigurationManager, EntityId, EntityManager, Player,
};
use crate::rar::EntityUpdateContext;
use crate::rar::GameState;
use crate::rar::PlayerInputContext;

pub struct GameStateGame {
	entity_configuration_manager: EntityConfigurationManager,
	entity_manager:               EntityManager,
}

impl GameStateGame {
	pub fn new() -> Self {
		Self {
			entity_manager:               EntityManager::new(),
			entity_configuration_manager: EntityConfigurationManager::new(),
		}
	}
}

impl GameState for GameStateGame {
	fn setup(&mut self, system: &mut System) {
		self.entity_configuration_manager
			.load(system, "todo_filename");

		self.entity_configuration_manager
			.load_yaml(system, "player.entity_config.yaml");

		//		println!("\n\n---\n{:#?}", &self.entity_configuration_manager );
		//		todo!("die");

		self.entity_manager.setup();

		// add player
		let mut player = Player::new();
		player.setup(self.entity_configuration_manager.get_config("player"));
		player.set_input_context_index(0);
		player.respawn();
		self.entity_manager.add(Box::new(player));

		// add 2nd player
		let mut player = Player::new();
		player.setup(self.entity_configuration_manager.get_config("player"));
		player.set_input_context_index(1);
		player.respawn();
		self.entity_manager.add(Box::new(player));

		// add background
		let mut background = Background::new();
		background.setup(self.entity_configuration_manager.get_config("background"));
		self.entity_manager.add(Box::new(background));
	}
	fn teardown(&mut self) {
		self.entity_manager.teardown();
	}
	fn update(&mut self, wuc: &mut WindowUpdateContext) {
		let mut euc = EntityUpdateContext::new();

		let mut pic = PlayerInputContext::default();
		if wuc.is_key_pressed('a' as u8) {
			pic.is_left_pressed = true;
		}
		if wuc.is_key_pressed('d' as u8) {
			pic.is_right_pressed = true;
		}
		euc.add_player_input_context(pic);

		let mut pic = PlayerInputContext::default();
		if wuc.is_key_pressed('j' as u8) {
			pic.is_left_pressed = true;
		}
		if wuc.is_key_pressed('l' as u8) {
			pic.is_right_pressed = true;
		}
		euc.add_player_input_context(pic);

		euc = euc.set_time_step(wuc.time_step);

		for e in self.entity_manager.iter_mut() {
			e.update(&mut euc);
		}
	}
	fn render(&mut self, renderer: &mut Renderer) {
		for e in self.entity_manager.iter_mut() {
			e.render(renderer);
		}
	}
	fn name(&self) -> &str {
		"[GameState] Game"
	}
}
