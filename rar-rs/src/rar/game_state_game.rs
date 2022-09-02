use oml_game::math::Vector2;
use oml_game::renderer::debug_renderer::DebugRenderer;
use oml_game::renderer::Color;
use oml_game::renderer::Renderer;
use oml_game::system::System;
use oml_game::window::WindowUpdateContext;

use crate::rar::camera::Camera;
use crate::rar::effect_ids::EffectId;
use crate::rar::entities::entity::Entity;
use crate::rar::entities::{
	Background, EntityConfigurationManager, EntityId, EntityManager, Player,
};
use crate::rar::game_state::GameStateResponse;
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
	fixed_camera: Camera,
	use_fixed_camera: bool,
	total_time: f64,
	player_id: EntityId,
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
		/* Note: moved down, and controlled by spawn position on world map
		let mut player = Player::new();
		player.setup(self.entity_configuration_manager.get_config("player"));
		player.set_input_context_index(0);
		player.respawn();
		let player_id = self.entity_manager.add(Box::new(player));

		self.camera.follow_player_entity_id(player_id);
		*/

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

		self.world
			.generate_collider_layers("Collider", &["Tile Layer"].to_vec())?;
		// self.world.generate_collider_layers( "Collider", &[ "Tile Layer" ].to_vec() )?; // force error for testing

		let player_spawns = self
			.world
			.list_objects_in_layer_for_class("Player", "PlayerSpawn");
		for ps in player_spawns.iter() {
			dbg!(&ps);
		}

		for ps in player_spawns.iter() {
			dbg!(&ps);
			match ps.data() {
				map::ObjectData::Point { pos } => {
					// add player ... at spawn position
					let mut player = Player::new();
					player.setup(self.entity_configuration_manager.get_config("player"));
					player.set_input_context_index(0);
					player.set_spawn_pos(&pos.add(&Vector2::new(64.0 + 32.0, 64.0)));
					player.respawn();
					let player_id = self.entity_manager.add(Box::new(player));

					self.camera.follow_player_entity_id(player_id);
					self.player_id = player_id;
					break; // just one for now ;)
				},
				o => {
					println!("Ignoring invalid object type for Player Spawn {:?}", &o);
				},
			}
		}

		let camera_starts = self
			.world
			.list_objects_in_layer_for_class("CameraControl", "CameraStart");
		for cs in camera_starts.iter() {
			dbg!(&cs);
		}

		for cs in camera_starts.iter() {
			dbg!(&cs);
			match cs.data() {
				map::ObjectData::Point { pos } => {
					self.camera.set_pos(pos);
					self.camera.set_target_pos(pos);
					break; // just one for now ;)
				},
				o => {
					println!("Ignoring invalid object type for Camera Start {:?}", &o);
				},
			}
		}

		// :HACK:
		//self.camera.freeze();

		self.world_renderer.setup()?;
		self.world_renderer.enable_layer(
			"Tile Layer 1",
			LayerId::TileMap1 as u8,
			EffectId::Textured as u16,
		);
		/*
		self.world_renderer.enable_layer(
			"Tile Layer 2",
			LayerId::TileMap2 as u8,
			EffectId::TexturedDesaturated as u16,
		);
		*/

		Ok(())
	}
	fn teardown(&mut self) {
		self.world_renderer.teardown();
		self.entity_manager.teardown();
	}
	fn update(&mut self, wuc: &mut WindowUpdateContext) -> Vec<GameStateResponse> {
		let mut euc = EntityUpdateContext::new();

		self.total_time += wuc.time_step;
		if wuc.was_key_pressed('p' as u8) {
			self.camera.punch(5.0);
		}

		if wuc.was_key_pressed('[' as u8) {
			self.use_fixed_camera = !self.use_fixed_camera;
		}

		let mut pic = PlayerInputContext::default();
		if let Some(p) = self.entity_manager.get_as_mut::<Player>(self.player_id) {
			if p.is_alive() && wuc.is_key_pressed('t' as u8) {
				// t for terminate
				p.kill();
			} else if wuc.is_key_pressed('r' as u8) {
				p.respawn();
				self.camera.thaw();
			}
		}

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

		// :HACK: we really need a better place to calculate our aspect ratio fixed frame
		let scaling = 1024.0 / wuc.window_size.y;
		let frame_size = Vector2::new(scaling * wuc.window_size.x, 1024.0);
		self.camera.set_frame_size(&frame_size);
		self.camera.update(wuc.time_step, &self.entity_manager);

		self.fixed_camera.set_frame_size(&frame_size);
		self.fixed_camera
			.update(wuc.time_step, &self.entity_manager);

		self.world_renderer.update(wuc.time_step);

		Vec::new()
	}
	fn render(&mut self, renderer: &mut Renderer) {
		//		let mtx = Matrix44::identity();
		// :TODO: apply camera offset
		//		renderer.mul_matrix( &mtx );
		let active_camera = if self.use_fixed_camera {
			&self.fixed_camera
		} else {
			&self.camera
		};
		/*
		if !self.use_fixed_camera {
			// :TODO: cycle through all cameras
			/*
			renderer.add_translation_for_layer(LayerId::Player as u8, &self.camera.offset());
			renderer.add_scaling_for_layer(LayerId::Player as u8, self.camera.scale()); // :TODO: handle via MatrixStack

			renderer.add_translation_for_layer(LayerId::TileMap1 as u8, &self.camera.offset());
			renderer.add_scaling_for_layer(LayerId::TileMap1 as u8, self.camera.scale()); // :TODO: handle via MatrixStack
			renderer.add_translation_for_layer(
				LayerId::TileMap2 as u8,
				&self.camera.offset().scaled_vector2(&Vector2::new(1.5, 1.0)),
			);
			renderer.add_scaling_for_layer(LayerId::TileMap2 as u8, self.camera.scale()); // :TODO: handle via MatrixStack
			*/
		}
		*/
		for e in self.entity_manager.iter_mut() {
			e.render(renderer, active_camera);
		}

		self.world_renderer
			.render(renderer, active_camera, &self.world);
		//		renderer.pop_matrix();
	}

	fn render_debug(&mut self, debug_renderer: &mut DebugRenderer) {
		let r = if self.use_fixed_camera {
			32.0 / self.camera.scale()
		} else {
			32.0
		};
		let cam_fixed = if self.use_fixed_camera {
			"FIXED"
		} else {
			"FREE"
		};
		debug_renderer.add_text(
			&Vector2::new(0.0, 400.0),
			&cam_fixed,
			25.0,
			3.0,
			&Color::white(),
		);

		let active_camera = if self.use_fixed_camera {
			&self.fixed_camera
		} else {
			&self.camera
		};
		let offset = active_camera.offset();

		// camera info
		let cam_pos = self.camera.pos().add(&offset);
		debug_renderer.add_circle(&cam_pos, r, 3.0, &Color::white());
		//		let cam_pos_text = format!("{:+07.1} / {:+07.1}", self.camera.pos().x, self.camera.pos().y );
		let cam_pos_text = format!(
			"{:+07.0}/{:+07.0}",
			self.camera.pos().x,
			self.camera.pos().y
		);
		//		let cam_pos_text = format!("{:+07.0} / {:+07.0}", cam_pos.x, cam_pos.y );
		debug_renderer.add_text(
			&cam_pos.add(&Vector2::new(0.0, 50.0)),
			&cam_pos_text,
			25.0,
			3.0,
			&Color::white(),
		);

		for wm in self.world.maps() {
			if let Some(m) = wm.map() {
				for l in m.layers() {
					self.world_renderer.render_debug_layer_objects(
						debug_renderer,
						active_camera,
						l,
					);
				}
			}
		}
		let frame = self.camera.frame();
		let cam_frame_text = format!(
			"{:+07.0}/{:+07.0} {:4.0}X{:4.0}",
			frame.x(),
			frame.y(),
			frame.width(),
			frame.height(),
		);
		debug_renderer.add_text(
			&Vector2::new(0.0, 512.0 - 30.0),
			&cam_frame_text,
			25.0,
			3.0,
			&Color::white(),
		);

		// :HACK: we always want to see the moving camera's frame!
		let frame = if self.use_fixed_camera {
			frame
		} else {
			frame.with_offset(&offset)
		};
		debug_renderer.add_rectangle(&frame, 7.0, &Color::from_rgba(0.9, 0.8, 0.6, 0.8));
		let cam_frame_center = frame.center();
		let screen_center = Vector2::zero();

		debug_renderer.add_line(&screen_center, cam_frame_center, 3.0, &Color::white());
	}

	fn name(&self) -> &str {
		"[GameState] Game"
	}
}
