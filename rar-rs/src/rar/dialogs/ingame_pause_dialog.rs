use std::sync::Arc;

use oml_game::math::Vector2;
use oml_game::renderer::Color;
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
		UiGridBox::default()
			.with_padding(16.0)
			.with_column_count(2)
			.containerize()
			.with_name("Paused Buttons")
			.with_tag("paused_buttons")
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
					{ UiSpacer::new(&Vector2::new(64.0, 64.0), &Color::white()).containerize() },
					{
						UiButton::new("ui-button_back", &Vector2::new(64.0, 64.0))
							.containerize()
							.with_name("back")
							.with_fade_out(0.0)
							.with_fade_in(1.0)
					},
					{
						UiButton::new("ui-button_confirm_danger", &Vector2::new(64.0, 64.0))
							.containerize()
							.with_name("back_confirm")
							.with_tag("back_confirm/button")
							.with_fade_out(0.0)
						//.with_fade_in(1.0)
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
					UiHbox::new()
						.with_padding(16.0)
						.containerize()
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
									.with_tag("playpause/toggle")
									.with_fade_out(0.0)
									.with_fade_in(1.0)
								},
								{
									UiSpacer::new(&Vector2::new(64.0, 64.0), &Color::white())
										.containerize()
								},
							]
							.into(),
						),
					self.create_paused_box(),
				]
				.into(),
			)
	}

	fn update_playpause(
		&self,
		_uielement: &dyn UiElement,
		container_data: &mut UiElementContainerData,
		is_paused: bool,
	) {
		container_data.find_child_container_by_tag_mut_then("paused_buttons", &mut |container| {
			if is_paused {
				container.fade_in(1.0);
			} else {
				container.fade_out(1.0);
			}
		});
		container_data.find_child_by_tag_as_mut_element_then::<UiToggleButton>(
			"playpause/toggle",
			&|pptb| {
				//dbg!(&pptb);
				if is_paused {
					pptb.goto_a();
				} else {
					pptb.goto_b();
				}
			},
		);

		if !is_paused {
			container_data.find_child_container_by_tag_mut_then(
				"back_confirm/button",
				&mut |c| {
					c.fade_out(1.0);
				},
			);			
		}
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
							let uielement: &dyn UiElement = self;
							self.update_playpause(uielement, container, game.is_paused);
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
		container_data: &mut UiElementContainerData,
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
				"settings" => {
					debug!("settings button clicked");
					return Some(Box::new(UiEventResponseGenericMessage::new(
						"settings/toggle",
					)));
				},
				"back" => {
					debug!("back button clicked");
					container_data.find_child_container_by_tag_mut_then(
						"back_confirm/button",
						&mut |c| {
							c.toggle_fade(1.0);
						},
					);
				},
				"back_confirm" => {
					debug!("back confirm button clicked");
					return Some(Box::new(UiEventResponseGenericMessage::new("back")));
				},
				_ => {},
			},
			None => {},
		};
		Some(response)
	}
}
