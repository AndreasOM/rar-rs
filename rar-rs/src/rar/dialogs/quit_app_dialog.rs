use std::sync::Arc;

use oml_game::system::Data;
use oml_game::system::System;
use tracing::*;

use crate::rar::data::RarData;
use crate::ui::UiEventResponse;
use crate::ui::*;

#[derive(Debug)]
pub struct QuitAppDialog {
	data:      Option<Arc<dyn Data>>,
	container: Option<UiElementContainer>,
}

impl QuitAppDialog {
	pub fn new(system: &mut System, ui_element_factory: &UiElementFactory) -> Self {
		let container =
			UiElementContainer::from_config_asset(system, ui_element_factory, "quit_app_dialog");
		Self {
			data: system.data().as_ref().map(|data| Arc::clone(data)),
			container,
		}
	}
}

impl UiElement for QuitAppDialog {
	fn as_any(&self) -> &dyn std::any::Any {
		self
	}
	fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
		self
	}

	fn setup_within_container(&mut self, container_data: &mut UiElementContainerData) {
		if let Some(container) = self.container.take() {
			container_data.add_child_element_container(container);
		} else {
			panic!("No container for IngamePauseDialog");
		};
	}
	fn update(&mut self, container: &mut UiElementContainerData, _time_step: f64) {}
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
				"back" => {
					debug!("back button clicked");
					return Some(Box::new(UiEventResponseGenericMessage::new("back")));
				},
				"quit_confirm" => {
					debug!("quit confirm button clicked");
					return Some(Box::new(UiEventResponseGenericMessage::new("quit_confirm")));
				},
				_ => {},
			},
			None => {},
		};
		Some(response)
	}
}
