use std::sync::mpsc::Sender;

use oml_game::math::Vector2;
use tracing::*;

use crate::ui::{
	UiElement, UiElementContainerData, UiElementContainerHandle, UiEvent, UiEventResponse,
	UiEventResponseButtonClicked, UiImage,
};

#[derive(Debug, Default)]
pub struct UiButton {
	imagesize: Vector2,
	imagename: String,
	image:     Option<UiElementContainerHandle>,
}

impl UiButton {
	pub fn new(imagename: &str, imagesize: &Vector2) -> Self {
		Self {
			imagesize: *imagesize,
			imagename: imagename.to_owned(),
			image:     None,
		}
	}
}

impl UiElement for UiButton {
	fn type_name(&self) -> &str {
		"[UiButton]"
	}
	fn as_any(&self) -> &dyn std::any::Any {
		self
	}
	fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
		self
	}
	fn setup_within_container(&mut self, container: &mut UiElementContainerData) {
		let image = container.add_child_element(UiImage::new(&self.imagename, &self.imagesize));
		self.image = Some(image);
	}
	fn handle_ui_event(
		&mut self,
		container: &mut UiElementContainerData,
		_event: &UiEvent,
		_event_sender: &Sender<Box<dyn UiEventResponse>>,
	) -> Option<Box<dyn UiEventResponse>> {
		debug!("Button clicked");
		Some(Box::new(UiEventResponseButtonClicked::new(&container.name)))
	}
	fn preferred_size(&self) -> Option<&Vector2> {
		Some(&self.imagesize)
	}
}
