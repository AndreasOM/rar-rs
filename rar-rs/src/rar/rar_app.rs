//use oml_audio::fileloader::{FileLoader, FileLoaderFile};
use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::path::Path;
use std::path::PathBuf;
use std::rc::Rc;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::sync::Arc;

pub use oml_audio::fileloader::{FileLoader, FileLoaderFile};
use oml_audio::Audio;
use oml_audio::AudioBackend;
//use oml_game::system::audio_fileloader_system::*;
use oml_game::math::{Matrix44, Vector2};
use oml_game::renderer::debug_renderer;
use oml_game::renderer::debug_renderer::DebugRenderer;
use oml_game::renderer::Color;
use oml_game::renderer::Effect;
use oml_game::renderer::Renderer;
use oml_game::system::audio_fileloader_system::*;
//use oml_game::renderer::TextureAtlas;
use oml_game::system::filesystem::Filesystem;
use oml_game::system::filesystem_archive::FilesystemArchive;
use oml_game::system::filesystem_disk::FilesystemDisk;
use oml_game::system::filesystem_layered::FilesystemLayered;
use oml_game::system::System;
use oml_game::window::{Window, WindowUpdateContext};
use oml_game::App;
use tracing::*;

use crate::omscript::ScriptVm;
use crate::rar::data::AudioData;
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
	audio:            Box<dyn AudioBackend<oml_game::system::System>>,
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

	next_game_states:              VecDeque<GameStates>,
	debug_zoomed_out:              bool,
	debug_zoom_factor:             f32,
	screenshot_requested:          bool,
	screenshot_sequence_requested: bool,
	script_queue:                  VecDeque<String>,
	script_vm:                     ScriptVm,

	slow_skip:           u32,
	pause_update:        bool,
	slow_motion_divider: u32,
}

impl Default for RarApp {
	fn default() -> Self {
		let system = System::new();
		let game_states: HashMap<GameStates, Box<dyn GameState>> = HashMap::new();

		let (sound_tx, sound_rx) = std::sync::mpsc::channel();
		Self {
			renderer: None,
			audio: Audio::create_default(),
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
			debug_zoom_factor: 1.0,
			active_game_state: GameStates::Menu,
			game_states,
			next_game_states: VecDeque::new(),
			screenshot_requested: false,
			screenshot_sequence_requested: false,

			script_queue: VecDeque::new(),
			script_vm: ScriptVm::default(),
			slow_skip: 0,
			pause_update: false,
			slow_motion_divider: 1,
		}
	}
}
impl RarApp {
	pub fn new() -> Self {
		Self {
			..Default::default()
		}
	}

	pub fn queue_script(&mut self, script_name: &str) {
		self.script_queue.push_back(script_name.to_string());
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
			/*
			ui_debug_config.select("Menu", 3);
			ui_debug_config.select("Settings", 3);
			//ui_debug_config.select("Game", 3);
			ui_debug_config.select("Debug Collisions", 1);
			ui_debug_config.select("Paused Buttons", 3);
			*/
			ui_debug_config.select("World Selection Dialog Box", 5);

			ui_debug_config.set_mode(UiDebugConfigMode::None);
		});
	}
	pub fn register_ui_elements_with_factory(_ui_element_factory: &mut UiElementFactory) {
		// Note: here we could add game specific UiElements
		// ui_element_factory.register_producer_via_info(&crate::ui::UiButton::info());
	}

	fn render_trace<T: oml_game::telemetry::TelemetryEntry, F>(
		debug_renderer: &mut DebugRenderer,
		name: &str,
		line_width: f32,
		color: &Color,
		f: F,
	) where
		F: Fn(&T) -> f32,
	{
		const scale_x: f32 = 2.0;
		const offset_x: f32 = -768.0;
		const offset_y: f32 = -256.0;
		let vec = oml_game::DefaultTelemetry::get::<T>(name);
		let vec: Vec<Option<f32>> = vec.iter().map(|mt| mt.as_ref().map(|t| f(t))).collect();
		for (i, sey) in vec.windows(2).enumerate() {
			let sy = sey[0];
			let ey = sey[1];
			match (sy, ey) {
				(Some(sy), Some(ey)) => {
					let s = Vector2::new(i as f32 * scale_x + offset_x, sy + offset_y);
					let e = Vector2::new((i + 1) as f32 * scale_x + offset_x, ey + offset_y);

					debug_renderer.add_line(&s, &e, line_width, color);
				},
				_ => {},
			}
		}
	}

	fn render_trace_pairs<T: oml_game::telemetry::TelemetryEntry, F>(
		debug_renderer: &mut DebugRenderer,
		name: &str,
		line_width: f32,
		color: &Color,
		f: F,
	) where
		F: Fn((Option<&T>, Option<&T>)) -> Option<(f32, f32)>,
	{
		const scale_x: f32 = 2.0;
		const offset_x: f32 = -768.0;
		const offset_y: f32 = -256.0;
		let vec = oml_game::DefaultTelemetry::get::<T>(name);
		let vec: Vec<(usize, (f32, f32))> = vec
			.iter()
			.enumerate()
			.collect::<Vec<(usize, &Option<T>)>>()
			.windows(2)
			.filter_map(
				|w| {
					// .
					let o = f((w[0].1.as_ref(), w[1].1.as_ref()));
					o.map(|v| (w[0].0, v))
				},
				// .
			)
			.collect();

		for (i, y) in vec.iter() {
			let sy = y.0;
			let ey = y.1;
			let s = Vector2::new(*i as f32 * scale_x + offset_x, sy + offset_y);
			let e = Vector2::new((*i + 1) as f32 * scale_x + offset_x, ey + offset_y);

			debug_renderer.add_line(&s, &e, line_width, color);
		}
	}

	fn render_telemetry(debug_renderer: &mut DebugRenderer) {
		const scale_x: f32 = 2.0;
		const offset_x: f32 = -768.0;
		const offset_y: f32 = -256.0;
		Self::render_trace::<f32, _>(debug_renderer, "player.speed.y", 1.5, &Color::pal(0), |y| {
			*y
		});
		Self::render_trace::<f32, _>(
			debug_renderer,
			"player.speed.x",
			1.5,
			&Color::pal_next(),
			|y| *y,
		);
		Self::render_trace::<f32, _>(
			debug_renderer,
			"collision.#",
			1.5,
			&Color::pal_next(),
			|y| *y * 10.0,
		);

		Self::render_trace_pairs::<f64, _>(
			debug_renderer,
			"fast frame",
			1.5,
			&Color::white(),
			|t| match t {
				(Some(se), Some(ee)) => Some((*se as f32, *ee as f32)),
				(Some(se), None) => Some((*se as f32, *se as f32)),
				_ => None,
			},
		);

		Self::render_trace_pairs::<f64, _>(debug_renderer, "slow frame", 1.5, &Color::red(), |t| {
			match t {
				(Some(se), Some(ee)) => Some((*se as f32, *ee as f32)),
				(Some(se), None) => Some((*se as f32, *se as f32)),
				_ => None,
			}
		});
		// cardinals
		let vec: Vec<Option<String>> =
			oml_game::DefaultTelemetry::get::<String>("player.collision.cardinal");
		for (i, cardinal) in vec.iter().enumerate() {
			if let Some(c) = &cardinal {
				let (col, o) = match c.as_str() {
					"bottom" => (Color::green(), -64.0),
					"left" | "right" => (Color::blue(), 64.0),
					_ => continue,
				};
				let s = Vector2::new((i as f32) * scale_x + offset_x, 128.0 + offset_y + o);
				let e = Vector2::new((i as f32) * scale_x + offset_x, -128.0 + offset_y + o);

				debug_renderer.add_line(&s, &e, 1.5, &col);
			}
		}
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

		let mut lfs = FilesystemLayered::new();
		let doc_dir = System::get_document_dir("rar-rs");
		self.add_filesystem_disk(&mut lfs, &doc_dir, true);
		self.system.set_savegame_filesystem(Box::new(lfs));

		//self.audio = Audio::create_default();

		if let Some(data) = self.system.data() {
			match data.as_any().downcast_ref::<RarData>() {
				Some(data) => {
					data.audio
						.write()
						.and_then(|mut audio| {
							audio.backend_type = self.audio.backend_type().to_owned();
							Ok(())
						})
						.unwrap();
				},
				None => {},
			}
		}

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

		oml_game::DefaultTelemetry::enable();
		Ok(())
	}

	fn teardown(&mut self) {
		self.game_state().teardown();
	}
	fn is_done(&self) -> bool {
		self.is_done
	}
	fn update(&mut self, wuc: &mut WindowUpdateContext) -> anyhow::Result<()> {
		self.slow_skip += 1;
		if self.slow_skip >= self.slow_motion_divider {
			self.slow_skip = 0;
			self.pause_update = false;
		} else {
			self.pause_update = true;
		}

		if self.pause_update {
			//return Ok(());
			wuc.time_step = wuc.time_step() * 0.0;
		}
		// debug!("App update time step: {}", wuc.time_step());
		oml_game::DefaultTelemetry::update();

		let _timestep = self.audio.update();
		if !self.script_vm.is_script_running() {
			if let Some(script_name) = self.script_queue.pop_front() {
				self.script_vm
					.load(&mut self.system, &script_name)
					.expect("---->");
				self.script_vm.run();
				todo!();
			}
		} else {
			// intentionally skip tick on the frame we load
			self.script_vm.tick();
		}

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

		if wuc.was_key_pressed('i' as u8) {
			self.slow_motion_divider *= 2;
		}

		if wuc.was_key_pressed('u' as u8) {
			self.slow_motion_divider /= 2;
		}

		self.slow_motion_divider = self.slow_motion_divider.clamp(1, 16);

		if wuc.was_function_key_pressed(2) {
			if let Some(game_state) = self.game_states.get_mut(&self.active_game_state) {
				let yaml = game_state.ui_to_yaml_config_string();
				debug!("{}", &yaml);
				todo!();
			}
		}

		if wuc.was_function_key_pressed(5) {
			if let Some(game_state) = self.game_states.get_mut(&self.active_game_state) {
				game_state.reload(&mut self.system)?;
			}
		}

		if wuc.was_function_key_pressed(11) {
			self.screenshot_sequence_requested = true;
		}

		if wuc.was_function_key_pressed(12) {
			tracing::debug!("F12 -> Screenshot");
			self.screenshot_requested = true;
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
		if wuc.was_key_pressed('/' as u8) {
			self.debug_zoomed_out = !self.debug_zoomed_out;
		}

		if self.debug_zoomed_out && wuc.mouse_wheel_line_delta.y.abs() > 1.0 {
			debug!("mouse_wheel_line_delta {}", wuc.mouse_wheel_line_delta.y);
			// :TODO: use close to
			let factor = if wuc.is_modifier_pressed(oml_game::window::ModifierKey::Alt) {
				0.0001
			} else {
				0.001
			};
			self.debug_zoom_factor += wuc.mouse_wheel_line_delta.y * factor;
			if self.debug_zoom_factor >= 5.0 {
				self.debug_zoom_factor = 5.0;
			} else if self.debug_zoom_factor <= 0.001 {
				self.debug_zoom_factor = 0.001;
			}
			debug!("debug_zoom_factor {}", self.debug_zoom_factor);
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
		let time_step = if self.pause_update {
			//return;
			0.0
		} else {
			time_step
		};
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
					0.6 * self.debug_zoom_factor
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
					let mut debug_renderer = debug_renderer.borrow_mut();

					Self::render_telemetry(&mut debug_renderer);
				}

				if let Some(debug_renderer) = &*self.debug_renderer {
					let debug_renderer = debug_renderer.borrow();
					debug_renderer.render(renderer);
				}

				if self.screenshot_requested {
					const BUILD_DATETIME: &str = env!("BUILD_DATETIME");
					const VERSION: &str = env!("CARGO_PKG_VERSION");
					let filename = format!("screenshot-rar-rs-{}-{}", BUILD_DATETIME, VERSION);
					tracing::debug!("Screenshot requested {}", &filename);
					renderer.queue_screenshot(0, 1, Some(&filename));
				}
				self.screenshot_requested = false;
				if self.screenshot_sequence_requested {
					const BUILD_DATETIME: &str = env!("BUILD_DATETIME");
					const VERSION: &str = env!("CARGO_PKG_VERSION");
					let filename = format!("screenshot-rar-rs-{}-{}", BUILD_DATETIME, VERSION);
					renderer.queue_screenshot(0, 60, Some(&filename));
				}
				self.screenshot_sequence_requested = false;

				debug_renderer::debug_renderer_render(renderer);
				renderer.end_frame();
			},
			None => {},
		}
	}
}
