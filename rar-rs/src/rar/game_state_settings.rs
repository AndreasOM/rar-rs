use core::any::Any;
use std::sync::mpsc::{channel, Receiver, Sender};

use oml_game::math::Matrix32;
use oml_game::math::Vector2;
use oml_game::renderer::debug_renderer::DebugRenderer;
use oml_game::renderer::Renderer;
use oml_game::system::System;
use tracing::*;

use crate::rar::effect_ids::EffectId;
use crate::rar::game_state::GameStateResponse;
use crate::rar::layer_ids::LayerId;
use crate::rar::AppUpdateContext;
use crate::rar::AudioMessage;
use crate::rar::GameState;
use crate::ui::UiEventResponse;
use crate::ui::UiGravityBox;
use crate::ui::UiSystem;
use crate::ui::*;

#[derive(Debug)]
pub struct GameStateSettings {
	ui_system:               UiSystem,
	event_response_sender:   Sender<Box<dyn UiEventResponse>>,
	event_response_receiver: Receiver<Box<dyn UiEventResponse>>,
}

impl Default for GameStateSettings {
	fn default() -> Self {
		let (tx, rx) = channel();
		Self {
			ui_system:               UiSystem::default(),
			event_response_sender:   tx,
			event_response_receiver: rx,
		}
	}
}

impl GameStateSettings {
	pub fn new() -> Self {
		Self {
			..Default::default()
		}
	}
}

impl GameState for GameStateSettings {
	fn name(&self) -> &str {
		"[GameState] Settings"
	}
	fn as_any(&self) -> &(dyn Any + 'static) {
		self
	}
	fn as_any_mut(&mut self) -> &mut (dyn Any + 'static) {
		self
	}
	fn setup(&mut self, system: &mut System) -> anyhow::Result<()> {
		self.ui_system
			.setup(system, self.event_response_sender.clone())?;

		let label_size = Vector2::new(256.0, 96.0);
		const VERSION: &str = env!("CARGO_PKG_VERSION");
		const BUILD_DATETIME: &str = env!("BUILD_DATETIME");
		const GIT_COMMIT_HASH: &str = env!("GIT_COMMIT_HASH");

		let code_build_number = env!("CODE_BUILD_NUMBER");
		let base_data_build_number_path = "base_build_number.txt";
		let mut f = system
			.default_filesystem_mut()
			.open(base_data_build_number_path);
		let base_data_build_number = if f.is_valid() {
			f.read_as_string()
		} else {
			"[file not found]".to_string()
		};

		self.ui_system.set_root(
			UiGravityBox::new()
				.with_padding(16.0)
				.with_gravity(&Vector2::new(-1.0, 1.0))
				.containerize()
				.with_name("Game State Settings")
				.with_child_element_containers(
					[{
						UiHbox::new()
							.with_padding(16.0)
							.containerize()
							.with_name("Settings hBox")
							.with_child_element_containers(
								[
									{
										UiButton::new("ui-button_back", &Vector2::new(64.0, 64.0))
											.containerize()
											.with_name("back")
											.with_fade_out(0.0)
											.with_fade_in(1.0)
									},
									{
										UiVbox::new()
											.with_padding(16.0)
											.containerize()
											.with_name("Settings hBox")
											.with_child_element_containers(
												[
													{
														UiToggleButton::new(
															"ui-togglebutton_music_on",
															"ui-togglebutton_music_off",
															&Vector2::new(64.0, 64.0),
														)
														.containerize()
														.with_name("music/toggle")
														.with_fade_out(0.0)
														.with_fade_in(1.0)
													},
													{
														UiToggleButton::new(
															"ui-togglebutton_sound_on",
															"ui-togglebutton_sound_off",
															&Vector2::new(64.0, 64.0),
														)
														.containerize()
														.with_name("sound/toggle")
														.with_fade_out(0.0)
														.with_fade_in(1.0)
													},
												]
												.into(),
											)
									},
									{
										UiVbox::new()
											.with_padding(16.0)
											.containerize()
											.with_name("Settings hBox")
											.with_child_element_containers(
												[
													{
														UiLabel::new(
															&label_size,
															&format!("Version : {}", VERSION),
														)
														.containerize()
													},
													{
														UiLabel::new(
															&label_size,
															&format!(
																"Build at: {}",
																BUILD_DATETIME
															),
														)
														.containerize()
													},
													{
														UiLabel::new(
															&label_size,
															&format!(
																"Code Build#: {}",
																code_build_number
															),
														)
														.containerize()
													},
													{
														UiLabel::new(
															&label_size,
															&format!(
																"'base' data Build#: {}",
																base_data_build_number
															),
														)
														.containerize()
													},
													{
														UiLabel::new(
															&label_size,
															&format!(
																"Commit : {}",
																GIT_COMMIT_HASH
															),
														)
														.containerize()
													},
												]
												.into(),
											)
									},
								]
								.into(),
							)
					}]
					.into(),
				),
		);
		self.ui_system.layout();
		Ok(())
	}
	fn teardown(&mut self) {
		self.ui_system.teardown();
	}
	fn set_size(&mut self, size: &Vector2) {
		self.ui_system.set_size(size);
		self.ui_system.layout();
		if let Some(root) = self.ui_system.get_root_mut() {
			root.set_size(size);
			/* :TODO: maybe
			if let Some(mut gbox) = root.find_child_mut(&["Debug Navigation Dialog - Gravity Box"])
			{
				let mut gbox = gbox.borrow_mut();
				gbox.set_size(size);
			}
			*/
		}
	}
	fn update(&mut self, auc: &mut AppUpdateContext) -> Vec<GameStateResponse> {
		let mut responses = Vec::new();

		self.ui_system.update(auc);

		if let Some(root) = self.ui_system.get_root_mut() {
			if let Some(mut mtb) = root.find_child_mut(&[
				"Game State Settings",
				"Settings hBox",
				"Settings hBox",
				"music/toggle",
			]) {
				// debug!("Found music/toggle");
				let mut mtb = mtb.borrow_mut();
				let mtb = mtb.borrow_element_mut();
				match mtb.as_any_mut().downcast_mut::<UiToggleButton>() {
					Some(mtb) => {
						if auc.is_music_playing() {
							mtb.goto_a();
						} else {
							mtb.goto_b();
						}
					},
					None => panic!("{:?} isn't a UiToggleButton!", &mtb),
				};
			} else {
				root.dump_info();
				todo!("Fix path to music toggle button");
			}
		}

		if let Some(root) = self.ui_system.get_root_mut() {
			if let Some(mut stb) = root.find_child_mut(&[
				"Game State Settings",
				"Settings hBox",
				"Settings hBox",
				"sound/toggle",
			]) {
				// debug!("Found sound/toggle");
				let mut stb = stb.borrow_mut();
				let stb = stb.borrow_element_mut();
				match stb.as_any_mut().downcast_mut::<UiToggleButton>() {
					Some(stb) => {
						if auc.is_sound_enabled() {
							stb.goto_a();
						} else {
							stb.goto_b();
						}
					},
					None => panic!("{:?} isn't a UiToggleButton!", &stb),
				};
			} else {
				root.dump_info();
				todo!("Fix path to sound toggle button");
			}
		}

		// :TODO:
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
}
