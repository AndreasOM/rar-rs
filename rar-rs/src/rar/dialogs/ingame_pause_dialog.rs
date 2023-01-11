use std::sync::Arc;

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
	fn create_children(&self) -> UiElementContainer {
		UiElementContainer::from_yaml(
			"
type: UiGridBox
padding: 0
column_count: 1
children:
  - type: UiGridBox
    padding: 16
    column_count: 2
    fade:
      - out 0.0
    children:
      - type: UiToggleButton
        name: playpause/toggle
        tag: playpause/toggle
        images:
          - ui-button_play
          - ui-button_pause
        size: 64x64
        fade:
          - out 0.0
          - in 1.0
      - type: UiSpacer
        size: 64x64
  - type: UiGridBox
    padding: 16
    column_count: 2
    name: Paused Buttons
    tag: paused_buttons
    fade:
      - out 0.0
    children:
      - type: UiButton
        name: settings
        image: ui-button_settings
        size: 64x64
        fade:
          - out 0.0
          - in 1.0
      - type: UiSpacer
        size: 64x64
      - type: UiButton
        name: back
        image: ui-button_back
        size: 64x64
        fade:
          - out 0.0
          - in 1.0
      - type: UiButton
        name: back_confirm
        tag: back_confirm/button
        image: ui-button_confirm_danger
        size: 64x64
        fade:
          - out 0.0    
",
		)
	}

	fn update_playpause(
		&self,
		_uielement: &dyn UiElement,
		container_data: &mut UiElementContainerData,
		is_paused: bool,
	) {
		let found = container_data.find_child_container_by_tag_mut_then(
			"paused_buttons",
			&mut |container| {
				debug!("Found paused_buttons");
				if is_paused {
					container.fade_in(1.0);
				} else {
					container.fade_out(1.0);
				}
			},
		);
		if !found {
			warn!("Could't find paused_buttons");
			container_data.dump_info();
			panic!();
		}
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
			container_data.find_child_container_by_tag_mut_then("back_confirm/button", &mut |c| {
				c.fade_out(1.0);
			});
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
