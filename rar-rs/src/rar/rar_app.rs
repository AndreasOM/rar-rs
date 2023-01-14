use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::rc::Rc;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::sync::Arc;

use oml_audio::Audio;
//use oml_game::system::audio_fileloader_system::*;
use oml_game::math::{Matrix44, Vector2};
use oml_game::renderer::debug_renderer;
use oml_game::renderer::debug_renderer::DebugRenderer;
use oml_game::renderer::Color;
use oml_game::renderer::Effect;
use oml_game::renderer::Renderer;
//use oml_game::renderer::TextureAtlas;
use oml_game::system::filesystem::Filesystem;
use oml_game::system::filesystem_archive::FilesystemArchive;
use oml_game::system::filesystem_disk::FilesystemDisk;
use oml_game::system::filesystem_layered::FilesystemLayered;
use oml_game::system::System;
use oml_game::window::{Window, WindowUpdateContext};
use oml_game::App;
use tracing::*;

use crate::rar::data::RarData;
use crate::rar::effect_ids::EffectId;
use crate::rar::font_ids::FontId;
//use crate::rar::game_state::get_game_state_as_specific;
use crate::rar::game_state::get_game_state_as_specific_mut;
use crate::rar::game_state::get_game_state_response_data_as_specific;
use crate::rar::game_state_debug_collisions::GameStateDebugCollisions;
//use crate::rar::entities::entity::Entity;
//use crate::rar::entities::{EntityConfigurationManager, Player};
use crate::rar::game_state_game::GameStateGame;
use crate::rar::game_state_menu::GameStateMenu;
use crate::rar::game_state_settings::GameStateSettings;
use crate::rar::layer_ids::LayerId;
use crate::rar::AppUpdateContext;
use crate::rar::AudioMessage;
//use crate::rar::EntityUpdateContext;
use crate::rar::GameState;
use crate::rar::GameStateResponseDataSelectWorld;
use crate::rar::RarUiUpdateContext;
use crate::ui::UiElementFactory;
use crate::ui::{UiDebugConfig, UiDebugConfigMode};

#[derive(Debug, PartialEq, Hash, Eq)]
enum GameStates {
	Menu,
	Game,
	DebugCollisions,
	Settings,
}

#[derive(Debug)]
pub struct RarApp {
	renderer:         Option<Renderer>,
	audio:            Audio,
	is_sound_enabled: bool,
	sound_rx:         Receiver<AudioMessage>,
	sound_tx:         Sender<AudioMessage>,

	size:           Vector2,
	viewport_size:  Vector2,
	scaling:        f32,
	system:         System,
	is_done:        bool,
	debug_renderer: Rc<Option<RefCell<DebugRenderer>>>,
	cursor_pos:     Vector2,
	total_time:     f64,

	//	entity_configuration_manager: EntityConfigurationManager,
	//	player: Player,
	fun: Vec<Vector2>,

	//	game_state: Box<dyn GameState>,
	game_states:       HashMap<GameStates, Box<dyn GameState>>,
	active_game_state: GameStates,

	next_game_states: VecDeque<GameStates>,
	debug_zoomed_out: bool,
}

impl Default for RarApp {
	fn default() -> Self {
		let system = System::new();
		let game_states: HashMap<GameStates, Box<dyn GameState>> = HashMap::new();

		let (sound_tx, sound_rx) = std::sync::mpsc::channel();
		Self {
			renderer: None,
			audio: Audio::new(),
			is_sound_enabled: true,
			sound_rx,
			sound_tx,
			size: Vector2::zero(),
			viewport_size: Vector2::zero(),
			scaling: 1.0,
			system,
			is_done: false,
			debug_renderer: Rc::new(None),
			cursor_pos: Vector2::zero(),
			total_time: 0.0,

			// entity_configuration_manager: EntityConfigurationManager::new(),
			// player: Player::new(),
			fun: Vec::new(),
			//			game_state:       Box::new(GameStateGame::new()),
			//			game_state:       Box::new(GameStateMenu::new()),
			debug_zoomed_out: false,
			active_game_state: GameStates::Menu,
			game_states,
			next_game_states: VecDeque::new(),
		}
	}
}
impl RarApp {
	pub fn new() -> Self {
		Self {
			..Default::default()
		}
	}
	// :TODO: Consider moving this into game package
	fn add_filesystem_disk(&mut self, lfs: &mut FilesystemLayered, path: &str, enable_write: bool) {
		let datadir = if path.starts_with("/") {
			path.to_owned()
		} else {
			let cwd = std::env::current_dir().unwrap();
			let cwd = cwd.to_string_lossy();

			let datadir = format!("{}/{}", &cwd, &path);
			datadir
		};

		let mut dfs = FilesystemDisk::new(&datadir);
		if enable_write {
			dfs.enable_write();
		}

		lfs.add_filesystem(Box::new(dfs));
	}
	// :TODO: Consider moving this into game package
	fn add_pakfile_from_file(&mut self, lfs: &mut FilesystemLayered, name: &str) -> bool {
		if let Some(p) = System::get_resource_path(name) {
			let base_dir = if p.starts_with("/") {
				// println!("Absolute");
				""
			} else {
				// println!("Relative");
				"."
			};
			let mut mfs = FilesystemDisk::new(base_dir);
			let mut omar_file = mfs.open(&p);

			if omar_file.is_valid() {
				let afs = FilesystemArchive::new_from_file(name, &mut omar_file);
				lfs.add_filesystem(Box::new(afs));

				true
			} else {
				println!("Broken pakfile {} from {:?}", &p, &mfs);
				false
			}
		} else {
			false
		}
	}

	fn game_state(&mut self) -> &mut Box<dyn GameState> {
		match self.game_states.get_mut(&self.active_game_state) {
			Some(gs) => return gs,
			None => {},
		}
		//		error!("Active GameState {:?} not in {:#?}", &self.active_game_state, &self.game_states );
		error!("Active GameState >{:?}< not found", &self.active_game_state);
		panic!("");
	}
	fn setup_debug(&mut self) {
		UiDebugConfig::write_then(&mut |ui_debug_config| {
			ui_debug_config.set_mode(UiDebugConfigMode::Selected);
			ui_debug_config.select("Menu", 3);
			ui_debug_config.select("Settings", 3);
			//ui_debug_config.select("Game", 3);
			ui_debug_config.select("Debug Collisions", 1);
			ui_debug_config.select("Paused Buttons", 3);

			ui_debug_config.set_mode(UiDebugConfigMode::None);
		});
	}
	pub fn register_ui_elements_with_factory(_ui_element_factory: &mut UiElementFactory) {
		// Note: here we could add game specific UiElements
		// ui_element_factory.register_producer_via_info(&crate::ui::UiButton::info());
	}
}

impl App for RarApp {
	fn remember_window_layout(&self) -> bool {
		true
	}
	fn app_name(&self) -> &str {
		"rar-rs"
	}

	fn setup(&mut self, window: &mut Window) -> anyhow::Result<()> {
		self.setup_debug();

		window.set_title("RAR - RS");

		let rar_data = RarData::new();
		self.system.set_data(Arc::new(rar_data));

		let game_states = &mut self.game_states;
		game_states.insert(GameStates::Menu, Box::new(GameStateMenu::new()));
		game_states.insert(
			GameStates::Game,
			Box::new(GameStateGame::new(&mut self.system)),
		);
		game_states.insert(
			GameStates::DebugCollisions,
			Box::new(GameStateDebugCollisions::new()),
		);
		game_states.insert(GameStates::Settings, Box::new(GameStateSettings::new()));

		let mut lfs = FilesystemLayered::new();

		// :TODO: handle linked in data?
		if self.add_pakfile_from_file(&mut lfs, "base.omar") {
			debug!("Using external archive");
		} else {
			warn!("No archive used");
		}

		// check local files first for faster development (and easier modding)
		self.add_filesystem_disk(&mut lfs, "../data/base", false);

		println!("lfs: {:?}", &lfs);

		self.system.set_default_filesystem(Box::new(lfs));

		let mut something_file = self.system.default_filesystem_mut().open("something.txt");
		//		println!("sf: {:?}", &something_file);
		//		println!("valid?: {:?}", something_file.is_valid());
		//		println!("size: {:?}", something_file.size());
		let something = something_file.read_as_string();

		println!("Something: {}", &something);

		self.audio.load_sound_bank(&mut self.system, "base.omsb");

		self.audio.load_music_native(&mut self.system, "title");
		self.audio.start();
		self.audio.play_music();
		let mut renderer = Renderer::new();
		renderer.setup(window, &mut self.system)?;

		renderer.register_effect(Effect::create(
			&mut self.system,
			EffectId::Background as u16,
			"Background",
			"background_vs.glsl",
			"background_fs.glsl",
		));

		renderer.register_effect(Effect::create(
			&mut self.system,
			EffectId::Colored as u16,
			"Colored",
			"colored_vs.glsl",
			"colored_fs.glsl",
		));
		renderer.register_effect(Effect::create(
			&mut self.system,
			EffectId::Textured as u16,
			"Textured",
			"textured_vs.glsl",
			"textured_fs.glsl",
		));
		renderer.register_effect(Effect::create(
			&mut self.system,
			EffectId::ColoredTextured as u16,
			"ColoredTextured",
			"coloredtextured_vs.glsl",
			"coloredtextured_fs.glsl",
		));
		renderer.register_effect(Effect::create(
			&mut self.system,
			EffectId::FontColored as u16,
			"FontColored",
			"fontcolored_vs.glsl",
			"fontcolored_fs.glsl",
		));
		renderer.register_effect(Effect::create(
			&mut self.system,
			EffectId::TexturedDesaturated as u16,
			"Textured Desaturated",
			"textured_desaturated_vs.glsl",
			"textured_desaturated_fs.glsl",
		));

		//TextureAtlas::load_all(&mut self.system, &mut renderer, "player-atlas-%d");
		//TextureAtlas::load_all(&mut self.system, &mut renderer, "bg-title-atlas");
		//TextureAtlas::load_all(&mut self.system, &mut renderer, "tileset-default-%d");

		//		renderer.load_font(&mut self.system, FontId::Default as u8, "c64");
		renderer.load_font(&mut self.system, FontId::Default as u8, "vegur");
		renderer.load_font(&mut self.system, FontId::Mono as u8, "inconsolata");

		self.renderer = Some(renderer);

		//self.game_state().setup(&mut self.system)?;
		if let Some(game_state) = self.game_states.get_mut(&self.active_game_state) {
			game_state.setup(&mut self.system)?;
		}

		Ok(())
	}

	fn teardown(&mut self) {
		self.game_state().teardown();
	}
	fn is_done(&self) -> bool {
		self.is_done
	}
	fn update(&mut self, wuc: &mut WindowUpdateContext) -> anyhow::Result<()> {
		// debug!("App update time step: {}", wuc.time_step());

		let _timestep = self.audio.update();

		if let Some(next_game_state) = self.next_game_states.pop_front() {
			if let Some(old_game_state) = self.game_states.get_mut(&self.active_game_state) {
				old_game_state.teardown();
			}

			if let Some(new_game_state) = self.game_states.get_mut(&next_game_state) {
				new_game_state.setup(&mut self.system).map_err(|err| {
					debug!("Error during GameState::setup -> {:?}", &err);
					todo!();
				});
			}

			self.active_game_state = next_game_state;
		}
		self.total_time += wuc.time_step;

		if wuc.is_escape_pressed {
			self.is_done = true;
		}
		/*
		if wuc.mouse_buttons[0] {
			println!("Mouse pressed: {} {}", wuc.mouse_pos.x, wuc.mouse_pos.y);
		}
		*/

		// the generic DebugRenderer
		if wuc.was_key_pressed('\\' as u8) {
			debug_renderer::debug_renderer_toggle(
				LayerId::DebugRenderer as u8,
				EffectId::Colored as u16,
			);
		}

		if wuc.was_key_pressed('=' as u8) {
			UiDebugConfig::write_then(&mut |ui_debug_config| {
				ui_debug_config.cycle_mode();
			});
		}

		if wuc.was_function_key_pressed( 5 ) {
			if let Some(game_state) = self.game_states.get_mut(&self.active_game_state) {
				game_state.reload(&mut self.system)?;
			}
		}

		debug_renderer::debug_renderer_begin_frame();

		// the specific DebugRenderer
		if wuc.was_key_pressed(']' as u8) {
			if self.debug_renderer.is_none() {
				self.debug_renderer = Rc::new(Some(RefCell::new(DebugRenderer::new(
					LayerId::DebugRenderer as u8,
					EffectId::Colored as u16,
				))));
				println!("Enabled debug renderer");
			} else {
				self.debug_renderer = Rc::new(None);
				println!("Disabled debug renderer");
			}
		}

		if wuc.was_key_pressed('^' as u8) {
			self.debug_zoomed_out = !self.debug_zoomed_out;
		}

		self.viewport_size = wuc.window_size;

		let scaling = 1024.0 / self.viewport_size.y;
		self.scaling = 1.0 * scaling; // !!! Do not tweak here

		self.size.x = (self.scaling) * self.viewport_size.x;
		self.size.y = (self.scaling) * self.viewport_size.y;

		if let Some(debug_renderer) = &*self.debug_renderer {
			let mut debug_renderer = debug_renderer.borrow_mut();
			debug_renderer.begin_frame();

			/*
						debug_renderer.add_text(
							&Vector2::new(-500.0 * 0.0, -175.0),
							"0123456789",
							75.0,
							3.0,
							&Color::red(),
						);
						debug_renderer.add_text(
							&Vector2::new(-500.0 * 0.0, -175.0 - 100.0),
							"ABCDEFGHIJKLMNOPQRSTUVWXYZ",
							75.0,
							7.0,
							&Color::from_rgba(0.075, 0.075, 0.095, 1.0),
						);
						debug_renderer.add_text(
							&Vector2::new(-500.0 * 0.0 + 5.0, -175.0 - 100.0 + 5.0),
							"ABCDEFGHIJKLMNOPQRSTUVWXYZ",
							75.0,
							7.0,
							&Color::rainbow(self.total_time as f32 * 36.0 * 5.0),
						);
			*/
		}

		/*
				if let Some( debug_renderer ) = &*self.debug_renderer {
					let mut debug_renderer = debug_renderer.borrow_mut();
					debug_renderer.add_line( &Vector2::new( 1.0, 1.0 ), &Vector2::zero(), 3.0, &Color::white() );
				}
		*/
		self.cursor_pos.x = 0.5 * self.scaling * wuc.window_size.x * (2.0 * wuc.mouse_pos.x - 1.0);
		self.cursor_pos.y = 0.5 * self.scaling * wuc.window_size.y * (2.0 * wuc.mouse_pos.y - 1.0);

		if wuc.was_key_pressed('f' as u8) {
			self.fun.push(self.cursor_pos.clone());
		}

		if let Some(debug_renderer) = &*self.debug_renderer {
			let mut debug_renderer = debug_renderer.borrow_mut();
			let mut last_fun = None;

			for this_fun in &self.fun {
				if let Some(l) = last_fun {
					debug_renderer.add_line(&l, &this_fun, 3.0, &Color::white());
				}
				last_fun = Some(this_fun.clone());
			}
		}

		//

		let ruuc = RarUiUpdateContext::default();

		let mut auc = AppUpdateContext::new()
			.set_time_step(wuc.time_step)
			.set_cursor_pos(&self.cursor_pos)
			.set_wuc(&wuc)
			.set_sound_tx(self.sound_tx.clone())
			.with_is_music_playing(self.audio.is_music_playing())
			.with_is_sound_enabled(self.is_sound_enabled)
			.with_ui_update_context(Box::new(ruuc));

		if let Some(data) = self.system.data() {
			match data.as_any().downcast_ref::<RarData>() {
				Some(data) => {
					data.audio.write().and_then(|mut audio| {
						// could probably try_write here
						//debug!("is_sound_enabled {:?}", audio.is_sound_enabled);
						audio.is_music_enabled = self.audio.is_music_playing();
						audio.is_sound_enabled = self.is_sound_enabled;
						Ok(())
					});
				},
				None => {},
			}
		}

		if let Some(game_state) = self.game_states.get_mut(&self.active_game_state) {
			game_state.set_size(&self.size); // :TODO: only call on change;
		}

		let responses = self.game_state().update(&mut auc);

		for r in responses.iter() {
			match r.name() {
				"QuitApp" => {
					debug!("QuitApp");
					self.is_done = true;
				},
				"GotoMainMenu" => {
					debug!("GotoMainMenu");
					self.next_game_states.push_back(GameStates::Menu);
				},
				"GotoSettings" => {
					debug!("GotoSettings");
					self.next_game_states.push_back(GameStates::Settings);
				},
				"StartGame" => {
					debug!("StartGame");
					self.next_game_states.push_back(GameStates::Game);
				},
				"SelectWorld" => {
					debug!("SelectWorld");
					debug!("{:?}", &r);
					if let Some(data) = r.data() {
						if let Some(swd) = get_game_state_response_data_as_specific::<
							GameStateResponseDataSelectWorld,
						>(data)
						{
							debug!("data -> {:?}", &swd);
							debug!("SelectWorld -> {}", &swd.world());
							if let Some(gs) = self.game_states.get_mut(&GameStates::Game) {
								if let Some(gsg) =
									get_game_state_as_specific_mut::<GameStateGame>(gs)
								{
									gsg.select_world(&swd.world());
								}
							}
						}
					}
				},
				"DebugCollisions" => {
					debug!("DebugCollisions");
					self.next_game_states.push_back(GameStates::DebugCollisions);
				},
				o => {
					warn!("Unhandled GameStateResponse: >{}<", &o);
				},
			}
		}

		if let Some(renderer) = &mut self.renderer {
			renderer.update(&mut self.system);
		}

		// :HACK:
		if let Some(debug_renderer) = &*self.debug_renderer {
			let mut debug_renderer = debug_renderer.borrow_mut();
			//self.game_state().render_debug(&mut debug_renderer);
			if let Some(game_state) = self.game_states.get_mut(&self.active_game_state) {
				game_state.render_debug(&mut debug_renderer);
			}
		}

		if let Some(debug_renderer) = &*self.debug_renderer {
			let mut debug_renderer = debug_renderer.borrow_mut();
			debug_renderer.add_line(&self.cursor_pos, &Vector2::zero(), 3.0, &Color::white());
			debug_renderer.add_text(
				&self.cursor_pos.add(&Vector2::new(0.0, 20.0)),
				&format!("{:.0} {:.0}", &self.cursor_pos.x, &self.cursor_pos.y),
				16.0,
				3.0,
				&Color::white(),
			);
		}

		if let Some(debug_renderer) = &*self.debug_renderer {
			let mut debug_renderer = debug_renderer.borrow_mut();
			debug_renderer.end_frame();
		}

		// handle sound channel/queue

		while let Some(msg) = self.sound_rx.try_recv().ok() {
			match msg {
				AudioMessage::PlaySound(sound) => {
					println!("sound: {}", sound);
					if self.is_sound_enabled {
						self.audio.play_sound(&sound);
					}
				},
				AudioMessage::ToggleMusic => {
					debug!("Toggle music");
					if self.audio.is_music_playing() {
						self.audio.pause_music();
					} else {
						self.audio.play_music();
					}
				},
				AudioMessage::ToggleSound => {
					debug!("Toggle sound");
					if self.is_sound_enabled {
						self.is_sound_enabled = false;
					// :TODO: stop running sounds
					} else {
						self.is_sound_enabled = true;
					}
				},
				// _ => {},
			}
		}
		Ok(())
	}
	fn fixed_update(&mut self, time_step: f64) {
		//debug!("Fixed Update: {}", time_step);
		self.game_state().fixed_update(time_step);
	}
	fn render(&mut self) {
		// :TODO: if let ???
		match &mut self.renderer {
			Some(renderer) => {
				renderer.set_size(&self.size);
				renderer.set_viewport(&Vector2::zero(), &self.viewport_size);
				renderer.begin_frame();
				let color = Color::from_rgba(
					0.5 + 0.5 * (self.total_time * 0.5).sin() as f32,
					0.5,
					0.5,
					1.0,
				);
				renderer.clear(&color);

				//				let scaling = self.scaling * 0.5;
				let scaling = if !self.debug_zoomed_out {
					0.5
				} else {
					if let Some(debug_renderer) = &*self.debug_renderer {
						let mut debug_renderer = debug_renderer.borrow_mut();
						let w = self.size.x;
						let rect = (-0.5 * w, -512.0, w, 1024.0).into();
						debug_renderer.add_rectangle(&rect, 1.0, &Color::white());
					}
					0.6
				};

				//				dbg!(&scaling);
				let left = -self.size.x * scaling;
				let right = self.size.x * scaling;
				let top = self.size.y * scaling;
				let bottom = -self.size.y * scaling;
				let near = 1.0;
				let far = -1.0;

				//				dbg!(&top,&bottom);

				let mvp = Matrix44::ortho(left, right, bottom, top, near, far);

				//				dbg!(&mvp);

				renderer.set_mvp_matrix(&mvp);

				//self.game_state().render(renderer);
				if let Some(game_state) = self.game_states.get_mut(&self.active_game_state) {
					game_state.render(renderer);
				}

				if let Some(debug_renderer) = &*self.debug_renderer {
					let debug_renderer = debug_renderer.borrow();
					debug_renderer.render(renderer);
				}

				debug_renderer::debug_renderer_render(renderer);
				renderer.end_frame();
			},
			None => {},
		}
	}
}
