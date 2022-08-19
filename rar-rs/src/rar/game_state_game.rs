use oml_game::math::Vector2;
use oml_game::renderer::debug_renderer::DebugRenderer;
use oml_game::renderer::Color;
use oml_game::renderer::Renderer;
use oml_game::system::System;
use oml_game::window::WindowUpdateContext;

use crate::rar::camera::Camera;
use crate::rar::effect_ids::EffectId;
use crate::rar::entities::entity::Entity;
use crate::rar::entities::{Background, EntityConfigurationManager, EntityManager, Player};
use crate::rar::layer_ids::LayerId;
use crate::rar::map;
use crate::rar::{EntityUpdateContext, GameState, PlayerInputContext, World, WorldRenderer};

#[derive(Debug, Default)]
pub struct GameStateGame {
	entity_configuration_manager: EntityConfigurationManager,
	entity_manager: EntityManager,
	world: World,
	world_renderer: WorldRenderer,
	camera: Camera,
	use_fixed_camera: bool,
	total_time: f64,
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
			.load_yaml(system, "player.entity_config.yaml")?;

		//		println!("\n\n---\n{:#?}", &self.entity_configuration_manager );
		//		todo!("die");

		self.entity_manager.setup();

		// add player
		let mut player = Player::new();
		player.setup(self.entity_configuration_manager.get_config("player"));
		player.set_input_context_index(0);
		player.respawn();
		let player_id = self.entity_manager.add(Box::new(player));

		self.camera.follow_player_entity_id(player_id);

		// add 2nd player
		/* disabled for now
		let mut player = Player::new();
		player.setup(self.entity_configuration_manager.get_config("player"));
		player.set_input_context_index(1);
		player.respawn();
		self.entity_manager.add(Box::new(player));
		*/

		// add background
		let mut background = Background::new();
		background.setup(self.entity_configuration_manager.get_config("background"));
		self.entity_manager.add(Box::new(background));

		// load world
		self.world.load(system, "dev")?;
		self.world.load_all_maps(system)?;
		self.world.load_all_tilesets(system)?;

		self.world_renderer.setup()?;
		self.world_renderer.enable_layer(
			"Tile Layer 1",
			LayerId::TileMap1 as u8,
			EffectId::Textured as u16,
		);
		self.world_renderer.enable_layer(
			"Tile Layer 2",
			LayerId::TileMap2 as u8,
			EffectId::TexturedDesaturated as u16,
		);

		Ok(())
	}
	fn teardown(&mut self) {
		self.world_renderer.teardown();
		self.entity_manager.teardown();
	}
	fn update(&mut self, wuc: &mut WindowUpdateContext) {
		let mut euc = EntityUpdateContext::new();

		self.total_time += wuc.time_step;
		if wuc.was_key_pressed('p' as u8) {
			self.camera.punch(5.0);
		}

		if wuc.was_key_pressed('[' as u8) {
			self.use_fixed_camera = !self.use_fixed_camera;
		}

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

			self.camera
				.set_target_pos(&self.camera.pos().add(&Vector2::new(100.0, 0.0)));
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

		self.camera.update(wuc.time_step, &self.entity_manager);
	}
	fn render(&mut self, renderer: &mut Renderer) {
		//		let mtx = Matrix44::identity();
		// :TODO: apply camera offset
		//		renderer.mul_matrix( &mtx );
		if !self.use_fixed_camera {
			// :TODO: cycle through all cameras
			renderer.add_translation_for_layer(LayerId::Player as u8, &self.camera.offset());
			renderer.add_scaling_for_layer(LayerId::Player as u8, self.camera.scale()); // :TODO: handle via MatrixStack

			renderer.add_translation_for_layer(LayerId::TileMap1 as u8, &self.camera.offset());
			renderer.add_scaling_for_layer(LayerId::TileMap1 as u8, self.camera.scale()); // :TODO: handle via MatrixStack
			renderer.add_translation_for_layer(
				LayerId::TileMap2 as u8,
				&self.camera.offset().scaled_vector2(&Vector2::new(1.5, 1.0)),
			);
			renderer.add_scaling_for_layer(LayerId::TileMap2 as u8, self.camera.scale()); // :TODO: handle via MatrixStack
		}
		for e in self.entity_manager.iter_mut() {
			e.render(renderer);
		}

		self.world_renderer.render(renderer, &self.world);
		//		renderer.pop_matrix();
	}
	fn render_debug(&mut self, debug_renderer: &mut DebugRenderer) {
		let offset = if !self.use_fixed_camera {
			self.camera.offset()
		} else {
			Vector2::zero()
		};
		let r = if !self.use_fixed_camera {
			32.0
		} else {
			32.0 / self.camera.scale()
		};
		debug_renderer.add_circle(&self.camera.pos().add(&offset), r, 3.0, &Color::white());
		for wm in self.world.maps() {
			if let Some(m) = wm.map() {
				for l in m.layers() {
					//if l.name() == "CameraControl" {
					for o in l.objects() {
						dbg!(&o);
						match o.data() {
							map::ObjectData::Rectangle { rect } => {
								let mut rect = rect.clone();
								//								let offset = self.camera.scaled_vector2( &Vector2::new( -1.0, 1.0 ) );
								rect.offset(&offset);
								debug_renderer.add_rectangle(&rect, 3.0, &Color::white());
								debug_renderer.add_text(
									&rect.center().add(&Vector2::new(3.0, 3.0)),
									o.class(),
									40.0,
									5.0,
									&Color::black(),
								);
								debug_renderer.add_text(
									&rect.center(),
									o.class(),
									40.0,
									5.0,
									&Color::rainbow(self.total_time as f32 * 20.0),
								);
								//								debug_renderer.add_text(rect.pos(), o.class(), 50.0, 5.0, &Color::from_rgba( 0.75, 0.75, 0.95, 1.0 ));
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
