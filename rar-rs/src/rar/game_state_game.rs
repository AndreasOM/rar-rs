use std::any::Any;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::Arc;

use oml_game::math::Vector2;
use oml_game::renderer::debug_renderer;
use oml_game::renderer::debug_renderer::DebugRenderer;
use oml_game::renderer::Color;
use oml_game::renderer::Renderer;
use oml_game::system::Data;
use oml_game::system::System;
use tracing::*;

use crate::rar::camera::Camera;
use crate::rar::data::RarData;
use crate::rar::dialogs::IngamePauseDialog;
use crate::rar::dialogs::SettingsDialog;
use crate::rar::effect_ids::EffectId;
use crate::rar::entities::entity::Entity;
use crate::rar::entities::{
	Background, EntityConfigurationManager, EntityId, EntityManager, Player,
};
use crate::rar::game_state::GameStateResponse;
use crate::rar::layer_ids::LayerId;
use crate::rar::map;
use crate::rar::AppUpdateContext;
//use oml_game::window::WindowUpdateContext;
use crate::rar::AudioMessage;
use crate::rar::{EntityUpdateContext, GameState, PlayerInputContext, World, WorldRenderer};
use crate::ui::UiElement;
use crate::ui::UiEventResponse;
use crate::ui::UiEventResponseButtonClicked;
use crate::ui::UiEventResponseGenericMessage;
use crate::ui::UiSystem;

#[derive(Debug)]
pub struct GameStateGame {
	ui_system: UiSystem,
	event_response_sender: Sender<Box<dyn UiEventResponse>>,
	event_response_receiver: Receiver<Box<dyn UiEventResponse>>,
	entity_configuration_manager: EntityConfigurationManager,
	entity_manager: EntityManager,
	world: World,
	world_renderer: WorldRenderer,
	camera: Camera,
	fixed_camera: Camera,
	use_fixed_camera: bool,
	total_time: f64,
	player_id: EntityId,
	world_name: String,
	fixed_update_count: u32,
	is_game_paused: bool,
	data: Option<Arc<dyn Data>>,
}

impl Default for GameStateGame {
	fn default() -> Self {
		let (tx, rx) = channel();
		Self {
			ui_system: UiSystem::default(),
			event_response_sender: tx,
			event_response_receiver: rx,
			camera: Default::default(),
			entity_configuration_manager: Default::default(),
			entity_manager: Default::default(),
			fixed_camera: Default::default(),
			fixed_update_count: Default::default(),
			player_id: Default::default(),
			total_time: Default::default(),
			use_fixed_camera: Default::default(),
			world: Default::default(),
			world_name: Default::default(),
			world_renderer: Default::default(),
			data: Default::default(),
			is_game_paused: false,
		}
	}
}

impl GameStateGame {
	pub fn new(system: &mut System) -> Self {
		Self {
			entity_manager: EntityManager::new(),
			entity_configuration_manager: EntityConfigurationManager::new(),
			world: World::new(),
			world_name: "dev".to_string(),
			data: system.data().as_ref().map(|data| Arc::clone(&data)),
			..Default::default()
		}
	}
	pub fn select_world(&mut self, world: &str) {
		self.world_name = world.to_string();
	}
	fn update_ui_system(
		&mut self,
		auc: &mut AppUpdateContext,
		responses: &mut Vec<GameStateResponse>,
	) {
		self.ui_system.update(auc);

		while let Ok(ev) = self.event_response_receiver.try_recv() {
			if let Some(gme) = ev.as_any().downcast_ref::<UiEventResponseGenericMessage>() {
				match gme.message.as_str() {
					"playpause/toggle" => {
						self.toggle_game_pause();
					},
					"settings/toggle" => {
						self.ui_system
							.toggle_child_fade(&["Ingame Settings Dialog"]);
					},
					"back" => {
						let r = GameStateResponse::new("GotoMainMenu");
						responses.push(r);
					},
					_ => {
						warn!("Unhandled generic message {}", &gme.message);
					},
				}
			} else if let Some(bce) = ev.as_any().downcast_ref::<UiEventResponseButtonClicked>() {
				println!("Button {} clicked", &bce.button_name);
				if let Some(sound_tx) = auc.sound_tx() {
					let _ = sound_tx.send(AudioMessage::PlaySound("BUTTON".to_string()));
				}
				match bce.button_name.as_str() {
					"music/toggle" => {
						if let Some(sound_tx) = auc.sound_tx() {
							let _ = sound_tx.send(AudioMessage::ToggleMusic);
						}
					},
					"sound/toggle" => {
						if let Some(sound_tx) = auc.sound_tx() {
							let _ = sound_tx.send(AudioMessage::ToggleSound);
						}
					},
					o => {
						println!("Unhandled button click from {}", o);
					},
				}
			}
			/*
			match ev.as_any().downcast_ref::<UiEventResponseButtonClicked>() {
				Some(e) => {
					println!("Button {} clicked", &e.button_name);
					if let Some(sound_tx) = auc.sound_tx() {
						let _ = sound_tx.send(AudioMessage::PlaySound("BUTTON".to_string()));
					}

					match e.button_name.as_str() {
						"music/toggle" => {
							if let Some(sound_tx) = auc.sound_tx() {
								let _ = sound_tx.send(AudioMessage::ToggleMusic);
							}
						},
						"sound/toggle" => {
							if let Some(sound_tx) = auc.sound_tx() {
								let _ = sound_tx.send(AudioMessage::ToggleSound);
							}
						},
						_ => {
							println!("Unhandled button click from {}", &e.button_name);
						},
					}
				},
				None => {},
			}

			*/
		}
	}

	fn toggle_game_pause(&mut self) {
		if self.is_game_paused {
			self.is_game_paused = false;
		} else {
			self.is_game_paused = true;
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
		debug!("Loading world {}", &self.world_name);
		self.world.load(system, &self.world_name)?;
		debug!("Loading all maps...");
		self.world.load_all_maps(system)?;
		debug!("Loading all tilesets...");
		self.world.load_all_tilesets(system)?;

		debug!("Generating colliders...");
		self.world
			.generate_collider_layers("Collider", &["Tile Layer 1", "terrain"].to_vec())?;
		//			.generate_collider_layers("Collider", &["Tile Layer", "terrain"].to_vec())?;
		//			.generate_collider_layers("Collider", &["Tile Layer"].to_vec())?;
		//		self.world
		//			.generate_collider_layers("Collider", &["terrain"].to_vec())?;
		// self.world.generate_collider_layers( "Collider", &[ "Tile Layer" ].to_vec() )?; // force error for testing

		/*
		let colliders = self
			.world
			.list_objects_in_layer("Collider");
		for c in colliders.iter() {
			dbg!(&c);
		}

		todo!("");
		*/

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
		self.world_renderer.enable_layer(
			"terrain",
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
		self.ui_system
			.setup("Game", system, self.event_response_sender.clone())?;

		//self.ui_system.set_root(
		self.ui_system.add_child(
			&Vector2::new(-1.0, 1.0),
			IngamePauseDialog::new(system)
				.containerize()
				.with_name("Ingame Pause Dialog"),
		);

		self.ui_system.add_child(
			&Vector2::new(0.0, 0.0),
			SettingsDialog::new(system)
				.containerize()
				.with_name("Ingame Settings Dialog")
				.with_fade_out(0.0),
		);

		self.ui_system.layout();

		Ok(())
	}
	fn teardown(&mut self) {
		self.ui_system.teardown();
		self.world_renderer.teardown();
		self.entity_manager.teardown();
	}
	fn set_size(&mut self, size: &Vector2) {
		self.ui_system.set_size(size);
		//self.ui_system.layout();
		// :TODO-UI:
		/*
		if let Some(root) = self.ui_system.get_root_mut() {
			root.set_size(size);
			if let Some(mut gbox) = root.find_child_mut(&["Ingame Pause Dialog - Gravity Box"]) {
				let mut gbox = gbox.borrow_mut();
				gbox.set_size(size);
			}
		}
		*/
	}

	fn update(&mut self, auc: &mut AppUpdateContext) -> Vec<GameStateResponse> {
		let mut response = Vec::new();
		self.fixed_update_count = 0;

		if let Some(data) = &self.data {
			match data.as_any().downcast_ref::<RarData>() {
				Some(data) => {
					data.game
						.write()
						.and_then(|mut game| {
							// could probably try_write here
							game.is_game_paused = self.is_game_paused;
							Ok(())
						})
						.unwrap();
				},
				None => {
					warn!("Could not update game data!");
				},
			}
		}

		self.update_ui_system(auc, &mut response);
		// :HACK: make debug rendering relative to camera
		{
			let active_camera = if self.use_fixed_camera {
				&self.fixed_camera
			} else {
				&self.camera
			};
			let offset = active_camera.offset();
			debug_renderer::debug_renderer_set_offset(&offset);
		}
		let wuc = match auc.wuc() {
			Some(wuc) => wuc,
			None => return Vec::new(),
		};
		// :HACK: we really need a better place to calculate our aspect ratio fixed frame
		let scaling = 1024.0 / wuc.window_size.y;
		let frame_size = Vector2::new(scaling * wuc.window_size.x, 1024.0);
		self.camera.set_frame_size(&frame_size);
		self.fixed_camera.set_frame_size(&frame_size);

		if wuc.was_key_pressed('p' as u8) {
			//self.camera.punch(5.0);
			self.toggle_game_pause();
		}

		if !self.is_game_paused {
			let mut euc = EntityUpdateContext::new().with_world(&self.world);

			self.total_time += wuc.time_step;

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

			self.camera.update(wuc.time_step, &self.entity_manager);

			self.fixed_camera
				.update(wuc.time_step, &self.entity_manager);

			self.world_renderer.update(wuc.time_step);
		}

		while let Ok(ev) = self.event_response_receiver.try_recv() {
			debug!("{:?}", &ev);
			match ev.as_any().downcast_ref::<UiEventResponseButtonClicked>() {
				Some(e) => {
					println!("Button {} clicked", &e.button_name);
					if let Some(sound_tx) = auc.sound_tx() {
						let _ = sound_tx.send(AudioMessage::PlaySound("BUTTON".to_string()));
					}

					match e.button_name.as_str() {
						"music/toggle" => {
							if let Some(sound_tx) = auc.sound_tx() {
								let _ = sound_tx.send(AudioMessage::ToggleMusic);
							}
						},
						"sound/toggle" => {
							if let Some(sound_tx) = auc.sound_tx() {
								let _ = sound_tx.send(AudioMessage::ToggleSound);
							}
						},
						_ => {
							println!("Unhandled button click from {}", &e.button_name);
						},
					}
				},
				None => {},
			}
		}
		response
	}
	fn fixed_update(&mut self, time_step: f64) {
		if !self.is_game_paused {
			let euc = EntityUpdateContext::new()
				.set_time_step(time_step)
				.with_fixed_update_count(self.fixed_update_count)
				.with_world(&self.world);

			for e in self.entity_manager.iter_mut() {
				e.fixed_update(&euc);
			}
		}
		self.fixed_update_count += 1;
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
		self.ui_system.render(renderer);
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
		self.ui_system.render_debug(debug_renderer);
	}

	fn name(&self) -> &str {
		"[GameState] Game"
	}
	fn as_any(&self) -> &(dyn Any + 'static) {
		self
	}
	fn as_any_mut(&mut self) -> &mut (dyn Any + 'static) {
		self
	}
}
