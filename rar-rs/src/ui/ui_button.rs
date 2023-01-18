use std::sync::mpsc::Sender;

use oml_game::math::Vector2;
use serde::Deserialize;
use tracing::*;

use crate::ui::UiElementInfo;
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

	pub fn from_yaml(yaml: &str) -> Self {
		let config: UiButtonConfig = serde_yaml::from_str(&yaml).unwrap();

		Self {
			imagesize: Vector2::from_x_str(&config.size),
			imagename: config.image,
			image:     None,
		}
	}
	pub fn info() -> &'static UiElementInfo {
		&UiElementInfo {
			type_name:   "UiButton",
			producer_fn: &Self::produce,
		}
	}

	pub fn produce() -> Box<dyn UiElement> {
		Box::new(Self::default())
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
		if !container.is_visible() {
			return None;
		}
		// :TODO: check actual event type
		debug!("Button clicked");
		Some(Box::new(UiEventResponseButtonClicked::new(&container.name)))
	}
	fn preferred_size(&self) -> Option<&Vector2> {
		Some(&self.imagesize)
	}
	fn configure_from_yaml_value(&mut self, yaml_value: serde_yaml::Value) {
		let config: UiButtonConfig = serde_yaml::from_value(yaml_value).unwrap();

		self.imagesize = Vector2::from_x_str(&config.size);
		self.imagename = config.image;
		if self.image.is_some() {
			panic!("Can not reconfigure {}", self.type_name());
		}
		// self.image =    None; :TODO: handle reconfigure
	}
	/*
	fn configure_from_yaml(&mut self, yaml: &str) {
		let config: UiButtonConfig = serde_yaml::from_str(&yaml).unwrap();

		self.imagesize = Vector2::from_x_str(&config.size);
		self.imagename = config.image;
		if self.image.is_some() {
			panic!("Can not reconfigure {}", self.type_name());
		}
		// self.image =    None; :TODO: handle reconfigure
	}
	*/
}

#[derive(Debug, Deserialize)]
struct UiButtonConfig {
	image: String,
	size:  String,
}
