use oml_game::math::Vector2;
use oml_game::renderer::debug_renderer::DebugRenderer;
use oml_game::renderer::Color;
use oml_game::renderer::Renderer;
use oml_game::system::System;
use oml_game::window::WindowUpdateContext;

use crate::rar::camera::Camera;
use crate::rar::entities::entity::Entity;
use crate::rar::entities::{
	Background, EntityConfigurationManager, EntityId, EntityManager, Player,
};
use crate::rar::map;
use crate::rar::EntityUpdateContext;
use crate::rar::GameState;
use crate::rar::PlayerInputContext;
use crate::rar::World;

#[derive(Debug,Default)]
pub struct GameStateGame {
	entity_configuration_manager: EntityConfigurationManager,
	entity_manager: EntityManager,
	world: World,
	camera: Camera,
}

impl GameStateGame {
	pub fn new() -> Self {
		Self {
			entity_manager: EntityManager::new(),
			entity_configuration_manager: EntityConfigurationManager::new(),
			world: World::new(),
			..Default::default()
		}
	}
}

impl GameState for GameStateGame {
	fn setup(&mut self, system: &mut System) -> anyhow::Result<()> {
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

		// load world
		self.world.load(system, "dev")?;
		self.world.load_all_maps(system)?;

		Ok(())
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
		if wuc.is_key_pressed('w' as u8) {
			pic.is_up_pressed = true;
			// :HACK:

			self.camera.set_target_pos( &self.camera.pos().add( &Vector2::new( 100.0, 0.0 ) ) );
		}
		if wuc.is_key_pressed('s' as u8) {
			pic.is_down_pressed = true;
		}
		euc.add_player_input_context(pic);

		let mut pic = PlayerInputContext::default();
		if wuc.is_key_pressed('j' as u8) {
			pic.is_left_pressed = true;
		}
		if wuc.is_key_pressed('l' as u8) {
			pic.is_right_pressed = true;
		}
		if wuc.is_key_pressed('i' as u8) {
			pic.is_up_pressed = true;
		}
		if wuc.is_key_pressed('k' as u8) {
			pic.is_down_pressed = true;
		}
		euc.add_player_input_context(pic);

		euc = euc.set_time_step(wuc.time_step);

		for e in self.entity_manager.iter_mut() {
			e.update(&mut euc);
		}

		self.camera.update( wuc.time_step );
	}
	fn render(&mut self, renderer: &mut Renderer) {
		for e in self.entity_manager.iter_mut() {
			e.render(renderer);
		}
	}
	fn render_debug(&mut self, debug_renderer: &mut DebugRenderer) {
		let offset = self.camera.offset();
		debug_renderer.add_circle( &offset, 32.0, 3.0, &Color::white() );
		for wm in self.world.maps() {
			if let Some(m) = wm.map() {
				for l in m.layers() {
					//if l.name() == "CameraControl" {
					for o in l.objects() {
						match o.data() {
							map::ObjectData::Rectangle { rect } => {
								let mut rect = rect.clone();
//								let offset = self.camera.scaled_vector2( &Vector2::new( -1.0, 1.0 ) );
								rect.offset( &offset );
								debug_renderer.add_rectangle( &rect, 3.0, &Color::white() );
							},
							_ => {},
						}
					}
					//}
				}
			}
		}
	}

	fn name(&self) -> &str {
		"[GameState] Game"
	}
}
