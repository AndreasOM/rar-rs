use std::any::Any;
use std::sync::mpsc::{channel, Receiver, Sender};

use oml_game::math::Matrix32;
//use oml_game::math::Rectangle;
use oml_game::math::Vector2;
use oml_game::renderer::Renderer;
use oml_game::system::System;

//use tracing::*;
use crate::rar::dialogs::WorldSelectionDialog;
use crate::rar::effect_ids::EffectId;
use crate::rar::game_state::GameStateResponse;
use crate::rar::layer_ids::LayerId;
use crate::rar::AppUpdateContext;
use crate::rar::GameState;
//use crate::rar::GameStateResponseDataSelectWorld;
use crate::ui::UiElementContainer;
//use crate::ui::UiEvent;
use crate::ui::UiEventResponse;
//use crate::ui::UiRenderer;
use crate::ui::UiSystem;

#[derive(Debug)]
pub struct GameStateMenu {
	//	world_selection_dialog: Option< WorldSelectionDialog >,
	//world_selection_dialog:  Option<UiElementContainer>,
	ui_system:               UiSystem,
	event_response_sender:   Sender<Box<dyn UiEventResponse>>,
	event_response_receiver: Receiver<Box<dyn UiEventResponse>>,
	//buttons:                 Vec<(&'static str, Rectangle)>,
}

impl Default for GameStateMenu {
	fn default() -> Self {
		let (tx, rx) = channel();
		Self {
			//world_selection_dialog:  None,
			ui_system:               UiSystem::default(),
			event_response_sender:   tx,
			event_response_receiver: rx,
			/*
			buttons:                 [
			/*
				("dev", (-128.0, -64.0, 256.0, 64.0).into()),
				("debug", (-128.0, 64.0, 256.0, 64.0).into()),
			*/
			]
			.to_vec(),
			*/
		}
	}
}

impl GameStateMenu {
	pub fn new() -> Self {
		Default::default()
	}
}

impl GameState for GameStateMenu {
	fn setup(&mut self, system: &mut System) -> anyhow::Result<()> {
		self.ui_system.setup(system)?;

		let wsd = WorldSelectionDialog::new();

		let mut wsd_container = UiElementContainer::new(Box::new(wsd));
		wsd_container.set_name("World Selection Dialog");

		// :HACK:
		wsd_container.layout(&Vector2::zero());
		self.ui_system.set_root(wsd_container);
		//self.world_selection_dialog = Some(wsd_container);
		Ok(())
	}
	fn teardown(&mut self) {
		self.ui_system.teardown();
		/*
		if let Some(_wsd) = self.world_selection_dialog.take() {
			// :TODO: cleanup wsd if needed
		}
		*/
	}
	fn update(&mut self, auc: &mut AppUpdateContext) -> Vec<GameStateResponse> {
		let mut responses = Vec::new();

		self.ui_system.update(auc);
		/*
		let wuc = match auc.wuc() {
			Some(wuc) => wuc,
			None => return Vec::new(),
		};
		*/

		/*
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
		*/

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

		self.ui_system.render(renderer);
	}
	fn as_any(&self) -> &(dyn Any + 'static) {
		self
	}
	fn as_any_mut(&mut self) -> &mut (dyn Any + 'static) {
		self
	}
}
