use std::any::Any;
use std::sync::mpsc::{channel, Receiver, Sender};

use oml_game::math::Matrix32;
//use oml_game::math::Rectangle;
use oml_game::math::Vector2;
use oml_game::renderer::debug_renderer::DebugRenderer;
use oml_game::renderer::Renderer;
use oml_game::system::System;
use tracing::*;

use crate::rar::dialogs::QuitAppDialog;
use crate::rar::dialogs::WorldSelectionDialog;
use crate::rar::effect_ids::EffectId;
use crate::rar::game_state::GameStateResponse;
use crate::rar::layer_ids::LayerId;
use crate::rar::AppUpdateContext;
//use tracing::*;
use crate::rar::AudioMessage;
use crate::rar::GameState;
use crate::rar::GameStateResponseDataSelectWorld;
use crate::rar::RarApp;
//use crate::rar::GameStateResponseDataSelectWorld;
use crate::ui::UiElement;
use crate::ui::UiElementFactory;
//use crate::ui::UiEvent;
use crate::ui::UiEventResponse;
use crate::ui::UiEventResponseButtonClicked;
//use crate::ui::UiRenderer;
use crate::ui::UiSystem;

#[derive(Debug)]
pub struct GameStateMenu {
	ui_system:               UiSystem,
	event_response_sender:   Sender<Box<dyn UiEventResponse>>,
	event_response_receiver: Receiver<Box<dyn UiEventResponse>>,
	ui_element_factory:      UiElementFactory,
	display_confirm_quit_in: f64,
}

impl Default for GameStateMenu {
	fn default() -> Self {
		let (tx, rx) = channel();
		Self {
			ui_system:               UiSystem::default(),
			event_response_sender:   tx,
			event_response_receiver: rx,
			ui_element_factory:      UiElementFactory::default().with_standard_ui_elements(),
			display_confirm_quit_in: 0.0,
		}
	}
}

impl GameStateMenu {
	pub fn new() -> Self {
		Default::default()
	}

	fn load_ui(&mut self, system: &mut System) {
		self.ui_system.add_child(
			&Vector2::new(0.0, 0.0),
			WorldSelectionDialog::new(system, &self.ui_element_factory)
				.containerize()
				.with_name("World Selection Dialog")
				.with_tag("world_selection_dialog"),
		);

		self.ui_system.add_child(
			&Vector2::new(0.0, 0.0),
			QuitAppDialog::new(system, &self.ui_element_factory)
				.containerize()
				.with_name("Quit App Dialog")
				.with_tag("quit_app_dialog")
				.with_fade_out(0.0),
		);

		self.ui_system.layout();
	}
}

impl GameState for GameStateMenu {
	fn setup(&mut self, system: &mut System) -> anyhow::Result<()> {
		RarApp::register_ui_elements_with_factory(&mut self.ui_element_factory);

		self.ui_system
			.setup("Menu", system, self.event_response_sender.clone())?;
		self.load_ui(system);

		Ok(())
	}
	fn teardown(&mut self) {
		self.ui_system.teardown();
	}
	fn set_size(&mut self, size: &Vector2) {
		self.ui_system.set_size(size);
		//self.ui_system.layout();
		//self.ui_system.dump_info();
		//todo!();
	}
	fn reload(&mut self, system: &mut System) -> anyhow::Result<()> {
		self.ui_system.teardown();
		self.ui_system
			.setup("Menu", system, self.event_response_sender.clone())?;
		self.load_ui(system);
		Ok(())
	}

	fn update(&mut self, auc: &mut AppUpdateContext) -> Vec<GameStateResponse> {
		let mut responses = Vec::new();

		self.ui_system.update(auc);

		if self.display_confirm_quit_in > 0.0 {
			self.display_confirm_quit_in -= auc.time_step();
			if self.display_confirm_quit_in <= 0.0 {
				self.ui_system.fade_in_child_by_tag("confirm_quit", 1.0);
			}
		}
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
						"dev" => {
							let world = "dev";
							let sw = GameStateResponseDataSelectWorld::new(world);
							let r = GameStateResponse::new("SelectWorld").with_data(Box::new(sw));
							responses.push(r);
							let r = GameStateResponse::new("StartGame");
							responses.push(r);
						},
						"grassland" => {
							let world = "grassland";
							let sw = GameStateResponseDataSelectWorld::new(world);
							let r = GameStateResponse::new("SelectWorld").with_data(Box::new(sw));
							responses.push(r);
							let r = GameStateResponse::new("StartGame");
							responses.push(r);
						},
						"mystic_mountain" => {
							let world = "mystic_mountain";
							let sw = GameStateResponseDataSelectWorld::new(world);
							let r = GameStateResponse::new("SelectWorld").with_data(Box::new(sw));
							responses.push(r);
							let r = GameStateResponse::new("StartGame");
							responses.push(r);
						},
						"DebugCollisions" => {
							let r = GameStateResponse::new("DebugCollisions");
							responses.push(r);
						},
						"Settings" => {
							let r = GameStateResponse::new("GotoSettings");
							responses.push(r);
						},
						"Quit" => {
							self.ui_system
								.fade_out_child_by_tag("world_selection_dialog", 1.0);
							self.ui_system.fade_in_child_by_tag("quit_app_dialog", 1.0);
							self.ui_system.fade_out_child_by_tag("confirm_quit", 0.0);
							self.ui_system.fade_out_child_by_tag("back", 0.0);
							self.ui_system.fade_in_child_by_tag("back", 0.5);
							self.display_confirm_quit_in = 1.0;
						},
						"Back" => {
							self.ui_system.fade_out_child_by_tag("quit_app_dialog", 1.0);
							self.ui_system
								.fade_in_child_by_tag("world_selection_dialog", 1.0);
							self.ui_system.fade_out_child_by_tag("confirm_quit", 0.5);
							self.ui_system.fade_out_child_by_tag("back", 0.5);
						},
						"Confirm Quit" => {
							let r = GameStateResponse::new("QuitApp");
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
	fn render_debug(&mut self, debug_renderer: &mut DebugRenderer) {
		self.ui_system.render_debug(debug_renderer);
	}

	fn ui_to_yaml_config(&self) -> serde_yaml::Value {
		self.ui_system.to_yaml_config()
	}

	fn ui_to_yaml_config_string(&self) -> String {
		self.ui_system.to_yaml_config_string()
	}

	fn as_any(&self) -> &(dyn Any + 'static) {
		self
	}
	fn as_any_mut(&mut self) -> &mut (dyn Any + 'static) {
		self
	}
}
