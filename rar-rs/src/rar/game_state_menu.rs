use std::any::Any;
use std::sync::mpsc::{channel, Receiver, Sender};

use oml_game::math::Matrix32;
use oml_game::math::Rectangle;
use oml_game::math::Vector2;
use oml_game::renderer::Renderer;
use oml_game::system::System;
use tracing::*;

use crate::rar::dialogs::WorldSelectionDialog;
use crate::rar::effect_ids::EffectId;
use crate::rar::game_state::GameStateResponse;
use crate::rar::layer_ids::LayerId;
use crate::rar::AppUpdateContext;
use crate::rar::GameState;
use crate::rar::GameStateResponseDataSelectWorld;
use crate::ui::UiElementContainer;
use crate::ui::UiEvent;
use crate::ui::UiEventResponse;
use crate::ui::UiRenderer;

#[derive(Debug)]
pub struct GameStateMenu {
	//	world_selection_dialog: Option< WorldSelectionDialog >,
	world_selection_dialog:  Option<UiElementContainer>,
	event_response_sender:   Sender<Box<dyn UiEventResponse>>,
	event_response_receiver: Receiver<Box<dyn UiEventResponse>>,
	buttons:                 Vec<(&'static str, Rectangle)>,
}

impl Default for GameStateMenu {
	fn default() -> Self {
		let (tx, rx) = channel();
		Self {
			world_selection_dialog:  None,
			event_response_sender:   tx,
			event_response_receiver: rx,
			buttons:                 [
			/*
				("dev", (-128.0, -64.0, 256.0, 64.0).into()),
				("debug", (-128.0, 64.0, 256.0, 64.0).into()),
			*/
			]
			.to_vec(),
		}
	}
}

impl GameStateMenu {
	pub fn new() -> Self {
		Default::default()
	}
}

impl GameState for GameStateMenu {
	fn setup(&mut self, _system: &mut System) -> anyhow::Result<()> {
		let wsd = WorldSelectionDialog::new();

		let mut wsd_container = UiElementContainer::new(Box::new(wsd));
		wsd_container.set_name("World Selection Dialog");

		wsd_container.layout(&Vector2::zero());
		self.world_selection_dialog = Some(wsd_container);
		Ok(())
	}
	fn teardown(&mut self) {
		if let Some(_wsd) = self.world_selection_dialog.take() {
			// :TODO: cleanup wsd if needed
		}
	}
	fn update(&mut self, auc: &mut AppUpdateContext) -> Vec<GameStateResponse> {
		let mut responses = Vec::new();

		let wuc = match auc.wuc() {
			Some(wuc) => wuc,
			None => return Vec::new(),
		};

		/*
				if wuc.was_mouse_button_pressed(0) {
					debug!("{}", auc.cursor_pos().y);
					for b in self.buttons.iter() {
						let r = &b.1;
						if r.contains(auc.cursor_pos()) {
							debug!("Clicked {}", &b.0);
							let world = b.0;
							let sw = GameStateResponseDataSelectWorld::new(world);
							let r = GameStateResponse::new("SelectWorld").with_data(Box::new(sw));
							responses.push(r);
							let r = GameStateResponse::new("StartGame");
							responses.push(r);
							continue;
						}
					}
					/*
								let world = if auc.cursor_pos().y < 0.0 {
									"dev"
								} else {
									"debug"
								};

					*/
				}
		*/
		if let Some(wsd) = &mut self.world_selection_dialog {
			if wuc.was_mouse_button_pressed(0) {
				let cp = auc.cursor_pos();
				debug!("Left Mouse Button was pressed @ {}, {}", cp.x, cp.y);
				let ev = UiEvent::MouseClick {
					pos:    *cp,
					button: 0,
				};
				if let Some(ev) = wsd.handle_ui_event(&ev, &self.event_response_sender) {
					println!("Click handled");
					self.event_response_sender.send(ev).unwrap();
				}
			}
			wsd.update(wuc.time_step());
		}

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

		renderer.use_texture("ui-button");
		renderer.use_layer(LayerId::Ui as u8);
		renderer.use_effect(EffectId::Textured as u16);

		for b in self.buttons.iter() {
			let r = &b.1;

			renderer.render_textured_quad(&r.center(), &r.size());
		}

		if let Some(wsd) = &mut self.world_selection_dialog {
			// :CHEAT: ???
			renderer.use_layer(LayerId::Ui as u8);
			//			renderer.use_effect( EffectId::ColoredTextured as u16 );

			let mut ui_renderer = UiRenderer::new(
				renderer,
				EffectId::ColoredTextured as u16,
				EffectId::Colored as u16,
				EffectId::FontColored as u16,
				LayerId::Ui as u8,
				LayerId::UiFront as u8,
			);
			wsd.render(&mut ui_renderer);
		}
	}
	fn as_any(&self) -> &(dyn Any + 'static) {
		self
	}
	fn as_any_mut(&mut self) -> &mut (dyn Any + 'static) {
		self
	}
}
