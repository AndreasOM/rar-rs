use std::any::Any;
use std::sync::mpsc::{channel, Receiver, Sender};

use oml_game::math::Matrix32;
//use oml_game::math::Rectangle;
use oml_game::math::Vector2;
use oml_game::math::Rectangle;
use oml_game::renderer::Color;
use oml_game::renderer::debug_renderer::DebugRenderer;
use oml_game::renderer::Renderer;
use oml_game::system::System;
use tracing::*;

//use tracing::*;

use crate::rar::effect_ids::EffectId;
use crate::rar::game_state::GameStateResponse;
use crate::rar::layer_ids::LayerId;
use crate::rar::AppUpdateContext;
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
	rectangles:					Vec< Rectangle >,
	target_pos:					Vector2,
}

impl Default for GameStateDebugCollisions {
	fn default() -> Self {
		let (tx, rx) = channel();
		Self {
			ui_system:               UiSystem::default(),
			event_response_sender:   tx,
			event_response_receiver: rx,
			rectangles: Vec::new(),
			target_pos:          Vector2::zero(),
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
		self.ui_system
			.setup(system, self.event_response_sender.clone())?;

		/*
		self.ui_system.set_root(
			WorldSelectionDialog::new()
				.containerize()
				.with_name("World Selection Dialog")
		);
		*/
		self.ui_system.layout();

		// add some rects
		self.rectangles.push( (50.0,50.0, 100.0,100.0).into() );
		self.rectangles.push( (50.0,-250.0, 100.0,100.0).into() );
		self.rectangles.push( (-150.0,50.0, 100.0,100.0).into() );
		self.rectangles.push( (-150.0,-250.0, 100.0,100.0).into() );
		Ok(())
	}
	fn teardown(&mut self) {
		self.ui_system.teardown();
	}
	fn set_size(&mut self, size: &Vector2) {
		self.ui_system.set_size(size);
	}

	fn update(&mut self, auc: &mut AppUpdateContext) -> Vec<GameStateResponse> {
		let mut responses = Vec::new();

		self.ui_system.update(auc);

		// :TODO: not sure if we actually want to pass events this far up
		/*
		for ev in self.event_response_receiver.try_recv() {
			debug!("{:?}", &ev);
			match ev.as_any().downcast_ref::<UiEventResponseButtonClicked>() {
				Some(e) => {
					println!("Button {} clicked", &e.button_name);
					match e.button_name.as_str() {
						"dev" => {
							let world = "dev";
							let sw = GameStateResponseDataSelectWorld::new(world);
							let r = GameStateResponse::new("SelectWorld").with_data(Box::new(sw));
							responses.push(r);
							let r = GameStateResponse::new("StartGame");
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
		*/

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
			let c = if r.contains( &self.target_pos ) {
				Color::blue()
			} else {
				Color::green()
			};
			debug_renderer.add_rectangle( &r, 3.0, &c );
		}

		debug_renderer.add_line( &Vector2::zero(), &self.target_pos, 5.0, &Color::red() );
	}

	fn as_any(&self) -> &(dyn Any + 'static) {
		self
	}
	fn as_any_mut(&mut self) -> &mut (dyn Any + 'static) {
		self
	}
}
