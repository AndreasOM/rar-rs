use std::cell::RefCell;
use std::rc::Rc;

use oml_game::math::{Matrix44, Vector2};
use oml_game::renderer::debug_renderer::DebugRenderer;
use oml_game::renderer::Color;
use oml_game::renderer::Effect;
use oml_game::renderer::Renderer;
use oml_game::renderer::TextureAtlas;
use oml_game::system::filesystem_disk::FilesystemDisk;
use oml_game::system::filesystem_layered::FilesystemLayered;
use oml_game::system::System;
use oml_game::window::{Window, WindowUpdateContext};
use oml_game::App;

use crate::rar::effect_ids::EffectId;
//use crate::rar::entities::entity::Entity;
//use crate::rar::entities::{EntityConfigurationManager, Player};
use crate::rar::game_state_game::GameStateGame;
use crate::rar::layer_ids::LayerId;
//use crate::rar::EntityUpdateContext;
use crate::rar::GameState;

pub struct RarApp {
	renderer:       Option<Renderer>,
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
	fun:        Vec<Vector2>,
	game_state: Box<dyn GameState>,
}

impl RarApp {
	pub fn new() -> Self {
		Self {
			renderer:       None,
			size:           Vector2::zero(),
			viewport_size:  Vector2::zero(),
			scaling:        1.0,
			system:         System::new(),
			is_done:        false,
			debug_renderer: Rc::new(None),
			cursor_pos:     Vector2::zero(),
			total_time:     0.0,

			// entity_configuration_manager: EntityConfigurationManager::new(),
			// player: Player::new(),
			fun:        Vec::new(),
			game_state: Box::new(GameStateGame::new()),
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
}

impl App for RarApp {
	fn setup(&mut self, window: &mut Window) -> anyhow::Result<()> {
		window.set_title("RAR - RS");

		let mut lfs = FilesystemLayered::new();
		self.add_filesystem_disk(&mut lfs, "../rar-data", false);

		println!("lfs: {:?}", &lfs);

		self.system.set_default_filesystem(Box::new(lfs));

		let mut something_file = self.system.default_filesystem_mut().open("something.txt");
		//		println!("sf: {:?}", &something_file);
		//		println!("valid?: {:?}", something_file.is_valid());
		//		println!("size: {:?}", something_file.size());
		let something = something_file.read_as_string();

		println!("Something: {}", &something);

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
			EffectId::TexturedDesaturated as u16,
			"Textured Desaturated",
			"textured_desaturated_vs.glsl",
			"textured_desaturated_fs.glsl",
		));

		TextureAtlas::load_all(&mut self.system, &mut renderer, "player-atlas-%d");
		TextureAtlas::load_all(&mut self.system, &mut renderer, "bg-title-atlas");
		TextureAtlas::load_all(&mut self.system, &mut renderer, "tileset-default-%d");

		self.renderer = Some(renderer);

		self.game_state.setup(&mut self.system)?;

		Ok(())
	}
	fn teardown(&mut self) {
		self.game_state.teardown();
	}
	fn is_done(&self) -> bool {
		self.is_done
	}
	fn update(&mut self, wuc: &mut WindowUpdateContext) {
		self.total_time += wuc.time_step;

		if wuc.is_escape_pressed {
			self.is_done = true;
		}
		if wuc.mouse_buttons[0] {
			println!("{} {}", wuc.mouse_pos.x, wuc.mouse_pos.y);
		}
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

		self.viewport_size = wuc.window_size;

		let scaling = 1024.0 / self.viewport_size.y;
		self.scaling = 1.0 * scaling; // !!! Do not tweak here

		self.size.x = (self.scaling) * self.viewport_size.x;
		self.size.y = (self.scaling) * self.viewport_size.y;

		if let Some(debug_renderer) = &*self.debug_renderer {
			let mut debug_renderer = debug_renderer.borrow_mut();
			debug_renderer.begin_frame();

			debug_renderer.add_text(&Vector2::new(0.0, 0.0), "TEST", 150.0, 5.0, &Color::green());
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
			//debug_renderer.add_text(&Vector2::new(0.0, 0.0), "T", &Color::green());
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

		self.game_state.update(wuc);

		// :HACK:
		if let Some(debug_renderer) = &*self.debug_renderer {
			let mut debug_renderer = debug_renderer.borrow_mut();
			self.game_state.render_debug(&mut debug_renderer);
		}

		if let Some(debug_renderer) = &*self.debug_renderer {
			let mut debug_renderer = debug_renderer.borrow_mut();
			debug_renderer.add_line(&self.cursor_pos, &Vector2::zero(), 3.0, &Color::white());
		}

		if let Some(debug_renderer) = &*self.debug_renderer {
			let mut debug_renderer = debug_renderer.borrow_mut();
			debug_renderer.end_frame();
		}
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
				let scaling = 0.5;
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

				self.game_state.render(renderer);

				if let Some(debug_renderer) = &*self.debug_renderer {
					let debug_renderer = debug_renderer.borrow();
					debug_renderer.render(renderer);
				}

				//debug_renderer::debug_renderer_render( renderer );
				renderer.end_frame();
			},
			None => {},
		}
	}
}
