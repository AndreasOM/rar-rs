use std::any::Any;
use std::sync::mpsc::{channel, Receiver, Sender};

use oml_game::math::Cardinals;
use oml_game::math::Matrix32;
use oml_game::math::Rectangle;
//use oml_game::math::Rectangle;
use oml_game::math::Vector2;
use oml_game::renderer::debug_renderer::DebugRenderer;
use oml_game::renderer::Color;
use oml_game::renderer::Renderer;
use oml_game::system::System;
use tracing::*;

use crate::rar::dialogs::DebugNavigationDialog;
use crate::rar::effect_ids::EffectId;
use crate::rar::game_state::GameStateResponse;
use crate::rar::layer_ids::LayerId;
use crate::rar::AppUpdateContext;
//use tracing::*;
use crate::rar::AudioMessage;
use crate::rar::GameState;
//use crate::rar::GameStateResponseDataSelectWorld;
use crate::ui::UiElement;
//use crate::ui::UiEvent;
use crate::ui::UiEventResponse;
use crate::ui::UiEventResponseButtonClicked;
//use crate::ui::UiRenderer;
use crate::ui::UiSystem;

#[derive(Debug)]
pub struct GameStateDebugCollisions {
	ui_system:               UiSystem,
	event_response_sender:   Sender<Box<dyn UiEventResponse>>,
	event_response_receiver: Receiver<Box<dyn UiEventResponse>>,
	rectangles:              Vec<Rectangle>,
	target_pos:              Vector2,
	test_rect:               Rectangle,
}

impl Default for GameStateDebugCollisions {
	fn default() -> Self {
		let (tx, rx) = channel();
		Self {
			ui_system:               UiSystem::default(),
			event_response_sender:   tx,
			event_response_receiver: rx,
			rectangles:              Vec::new(),
			target_pos:              Vector2::zero(),
			test_rect:               Rectangle::default().with_size(&Vector2::new(40.0, 80.0)),
		}
	}
}

impl GameStateDebugCollisions {
	pub fn new() -> Self {
		Default::default()
	}
}

impl GameState for GameStateDebugCollisions {
	fn setup(&mut self, system: &mut System) -> anyhow::Result<()> {
		self.ui_system.setup(
			"Debug Collisions",
			system,
			self.event_response_sender.clone(),
		)?;

		//self.ui_system.set_root(
		self.ui_system.add_child(
			&Vector2::new(-1.0, 1.0),
			DebugNavigationDialog::new()
				.containerize()
				.with_name("Debug Navigation Dialog"),
		);

		self.ui_system.layout();

		// add some rects
		self.rectangles.push((-50.0, -50.0, 100.0, 100.0).into());
		self.rectangles.push((50.0, 250.0, 100.0, 100.0).into());
		self.rectangles.push((250.0, 250.0, 500.0, 500.0).into());
		self.rectangles.push((-50.0, -250.0, 100.0, 100.0).into());
		self.rectangles.push((-150.0, 250.0, 100.0, 100.0).into());
		self.rectangles.push((-150.0, -250.0, 100.0, 100.0).into());
		self.rectangles.push((-500.0, -500.0, 1000.0, 200.0).into());
		self.rectangles.push((-850.0, 200.0, 200.0, 200.0).into());
		self.rectangles.push((-1050.0, 0.0, 200.0, 200.0).into());
		self.rectangles.push((-850.0, -400.0, 200.0, 200.0).into());
		self.rectangles.push((850.0, 200.0, 200.0, 200.0).into());
		self.rectangles.push((850.0, -400.0, 200.0, 200.0).into());
		Ok(())
	}
	fn teardown(&mut self) {
		self.ui_system.teardown();
	}
	fn set_size(&mut self, size: &Vector2) {
		self.ui_system.set_size(size);
		//self.ui_system.layout();
		//self.ui_system.dump_info();
		// :TODO-UI:
		/*
		if let Some(root) = self.ui_system.get_root_mut() {
			root.set_size(size);
			//			debug!("Set size of >{}< to {:?} => {:?}", root.name(), size, root.size());

			if let Some(mut gbox) = root.find_child_mut(&["Debug Navigation Dialog - Gravity Box"])
			{
				let mut gbox = gbox.borrow_mut();
				gbox.set_size(size);
			}
		}
		*/
	}

	fn update(&mut self, auc: &mut AppUpdateContext) -> Vec<GameStateResponse> {
		let mut responses = Vec::new();

		self.ui_system.update(auc);

		// :TODO: not sure if we actually want to pass events this far up
		for ev in self.event_response_receiver.try_recv() {
			debug!("{:?}", &ev);
			match ev.as_any().downcast_ref::<UiEventResponseButtonClicked>() {
				Some(e) => {
					println!("Button {} clicked", &e.button_name);
					if let Some(sound_tx) = auc.sound_tx() {
						let _ = sound_tx.send(AudioMessage::PlaySound("BUTTON".to_string()));
					}
					match e.button_name.as_str() {
						"back" => {
							let r = GameStateResponse::new("GotoMainMenu");
							responses.push(r);
						},
						_ => {
							println!("Unhandled button click from {}", &e.button_name);
						},
					}
				},
				None => {},
			};
		}

		self.target_pos = *auc.cursor_pos();

		responses
	}
	fn render(&mut self, renderer: &mut Renderer) {
		renderer.use_texture("bg-menu");
		renderer.use_layer(LayerId::Background as u8);
		renderer.use_effect(EffectId::Background as u16);

		let a = renderer.aspect_ratio();
		let mtx = Matrix32::scaling_xy(1.0 * a, 1.0);
		//mtx.pos.x = - self.pos.x / 1024.0;
		renderer.set_tex_matrix(&mtx);

		renderer.render_textured_fullscreen_quad();

		renderer.set_tex_matrix(&Matrix32::identity());

		self.ui_system.render(renderer);
	}
	fn render_debug(&mut self, debug_renderer: &mut DebugRenderer) {
		self.ui_system.render_debug(debug_renderer);

		for r in self.rectangles.iter() {
			let c = if r.contains(&self.target_pos) {
				Color::blue()
			} else {
				Color::green()
			};
			debug_renderer.add_rectangle(&r, 3.0, &c);
		}

		debug_renderer.add_line(&Vector2::zero(), &self.target_pos, 5.0, &Color::red());
		let target_rect = self.test_rect.clone().with_center(&self.target_pos);
		debug_renderer.add_rectangle(&target_rect, 3.0, &Color::white());

		// find first collision

		let mut first_col: Option<(f32, Cardinals)> = None;

		let start = Vector2::zero();
		let end = &self.target_pos.clone();

		for r in self.rectangles.iter() {
			if let Some(col) = r.would_collide(&start, &end, &self.test_rect) {
				let old_distance = if let Some(old_col) = first_col {
					old_col.0
				} else {
					f32::MAX
				};
				if col.0 < old_distance {
					first_col = Some(col);
				};
			}
		}

		if let Some(col) = first_col {
			let p = col.0;
			let full = end.sub(&start).scaled(p);
			let actual = start.add(&full);
			let actual_rect = self.test_rect.clone().with_center(&actual);
			debug_renderer.add_rectangle(&actual_rect, 3.0, &Color::white());
			debug_renderer.add_text(
				actual_rect.center(),
				&format!("{}", p),
				20.0,
				3.0,
				&Color::white(),
			);

			let l = match col.1 {
				Cardinals::Bottom => {
					let x0 = actual_rect.left();
					let x1 = actual_rect.right();
					let y = actual_rect.bottom();
					Some((Vector2::new(x0, y), Vector2::new(x1, y)))
				},
				Cardinals::Top => {
					let x0 = actual_rect.left();
					let x1 = actual_rect.right();
					let y = actual_rect.top();
					Some((Vector2::new(x0, y), Vector2::new(x1, y)))
				},
				Cardinals::Left => {
					let x = actual_rect.left();
					let y0 = actual_rect.bottom();
					let y1 = actual_rect.top();
					Some((Vector2::new(x, y0), Vector2::new(x, y1)))
				},
				Cardinals::Right => {
					let x = actual_rect.right();
					let y0 = actual_rect.bottom();
					let y1 = actual_rect.top();
					Some((Vector2::new(x, y0), Vector2::new(x, y1)))
				},
			};

			if let Some(l) = l {
				debug_renderer.add_line(&l.0, &l.1, 3.0, &Color::red());
			}
		}
	}

	fn as_any(&self) -> &(dyn Any + 'static) {
		self
	}
	fn as_any_mut(&mut self) -> &mut (dyn Any + 'static) {
		self
	}
}
