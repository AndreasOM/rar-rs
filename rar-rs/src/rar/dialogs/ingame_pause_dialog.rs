use std::sync::Arc;

use oml_game::math::Vector2;
use oml_game::system::Data;
use oml_game::system::System;
use tracing::*;

use crate::rar::data::RarData;
use crate::ui::UiEventResponse;
use crate::ui::*;

#[derive(Debug)]
pub struct IngamePauseDialog {
	data: Option<Arc<dyn Data>>,
}

impl IngamePauseDialog {
	pub fn new(system: &mut System) -> Self {
		Self {
			data: system.data().as_ref().map(|data| Arc::clone(data)),
		}
	}
	fn create_paused_box(&self) -> UiElementContainer {
		UiVbox::new()
			.with_padding(16.0)
			.containerize()
			.with_name("Paused Buttons")
			.with_fade_out(0.0)
			.with_child_element_containers(
				[
					{
						UiButton::new("ui-button_settings", &Vector2::new(64.0, 64.0))
							.containerize()
							.with_name("settings")
							.with_fade_out(0.0)
							.with_fade_in(1.0)
					},
					{
						UiButton::new("ui-button_back", &Vector2::new(64.0, 64.0))
							.containerize()
							.with_name("back")
							.with_fade_out(0.0)
							.with_fade_in(1.0)
					},
				]
				.into(),
			)
	}
	fn create_children(&self) -> UiElementContainer {
		UiVbox::new()
			.with_padding(16.0)
			.containerize()
			.with_name("Ingame Pause vBox")
			.with_child_element_containers(
				[
					{
						UiToggleButton::new(
							"ui-button_play",
							"ui-button_pause",
							&Vector2::new(64.0, 64.0),
						)
						.containerize()
						.with_name("playpause/toggle")
						.with_fade_out(0.0)
						.with_fade_in(1.0)
					},
					self.create_paused_box(),
					/*
										{
											UiButton::new("ui-button_back", &Vector2::new(64.0, 64.0))
												.containerize()
												.with_name("back")
												.with_fade_out(0.0)
												.with_fade_in(1.0)
										},
					*/
										/*
										{
											UiVbox::new()
												.with_padding(16.0)
												.containerize()
												.with_name("Settings hBox") // :TODO: fix name
												.with_child_element_containers(self.create_audio_buttons())
										},
										{
											UiHbox::new()
												.with_padding(16.0)
												.containerize()
												.with_name("Labels hBox")
												.with_child_element_containers(self.create_info_labels())
										},
										*/
				]
				.into(),
			)
	}

	fn update_playpause(
		&self,
		_uielement: &dyn UiElement,
		container: &mut UiElementContainerData,
		is_paused: bool,
	) {
		container.find_child_container_mut_then(
			&[
				//???"Settings hBox",
				//"Ingame Pause Dialog - Gravity Box",
				"Ingame Pause vBox",
				"Paused Buttons",
			],
			&|container| {
				//dbg!(&container);

				if is_paused {
					container.fade_in(1.0);
				} else {
					container.fade_out(1.0);
				}
			},
		);
		container.find_child_mut_as_element_then::<UiToggleButton>(
			&[
				//???"Settings hBox",
				//"Ingame Pause Dialog - Gravity Box",
				"Ingame Pause vBox",
				"playpause/toggle",
			],
			&|pptb| {
				//dbg!(&pptb);
				if is_paused {
					pptb.goto_a();
				} else {
					pptb.goto_b();
				}
			},
		);
		/* Example: without the helper
				if let Some(mut pptb) = container.find_child_mut(&[
					//???"Settings hBox",
					"Ingame Pause Dialog - Gravity Box",
					"Ingame Pause vBox",
					"playpause/toggle",
				]) {
					// debug!("Found sound/toggle");
					let mut pptb = pptb.borrow_mut();
					let pptb = pptb.borrow_element_mut();
					match pptb.as_any_mut().downcast_mut::<UiToggleButton>() {
						Some(pptb) => {
							if is_paused {
								pptb.goto_a();
							} else {
								pptb.goto_b();
							}
						},
						None => panic!("{:?} isn't a UiToggleButton!", &pptb),
					};
				} else {
					// ??? container.dump_info();
					todo!("Fix path to sound toggle button");
				}
		*/
	}
}

impl UiElement for IngamePauseDialog {
	fn as_any(&self) -> &dyn std::any::Any {
		self
	}
	fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
		self
	}

	fn setup_within_container(&mut self, container: &mut UiElementContainerData) {
		container.add_child_element_container(self.create_children());
	}
	fn update(&mut self, container: &mut UiElementContainerData, _time_step: f64) {
		if let Some(data) = &self.data {
			match data.as_any().downcast_ref::<RarData>() {
				Some(data) => {
					data.game
						.read()
						.and_then(|game| {
							// :TODO: maybe use try_read instead of potentially blocking
							//debug!("is_sound_enabled {:?}", audio.is_sound_enabled);
							//debug!("is_music_enabled {:?}", audio.is_music_enabled);
							let uielement: &dyn UiElement = self;
							self.update_playpause(uielement, container, game.is_game_paused);
							Ok(())
						})
						.unwrap();
				},
				None => {
					todo!();
				},
			}
		}
	}
	fn handle_ui_event_response(
		&mut self,
		response: Box<dyn UiEventResponse>,
	) -> Option<Box<dyn UiEventResponse>> {
		match response
			.as_any()
			.downcast_ref::<UiEventResponseButtonClicked>()
		{
			Some(bce) => match bce.button_name.as_str() {
				"playpause/toggle" => {
					debug!("playpause button clicked");
					return Some(Box::new(UiEventResponseGenericMessage::new(
						"playpause/toggle",
					)));
				},
				"back" => {
					debug!("back button clicked");
					return Some(Box::new(UiEventResponseGenericMessage::new("back")));
				},
				_ => {},
			},
			None => {},
		};
		Some(response)
	}
}
