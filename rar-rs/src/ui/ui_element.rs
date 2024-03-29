use std::sync::mpsc::Sender;

use oml_game::math::Rectangle;
use oml_game::math::Vector2;
use oml_game::renderer::debug_renderer::DebugRenderer;

//use oml_game::renderer::Color;
use crate::ui::{UiElementContainer, UiElementContainerData, UiEvent, UiEventResponse, UiRenderer};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct UiElementFadeData {
	pub level: f32,
	pub speed: f32,
}

#[derive(Debug, Copy, Clone, Default, PartialEq)]
pub enum UiElementFadeState {
	FadedOut,
	FadingIn(UiElementFadeData),
	#[default]
	FadedIn,
	FadingOut(UiElementFadeData),
}

pub trait UiElement {
	fn type_name(&self) -> &str {
		"[Trait] [UiElement]"
	}
	fn as_any(&self) -> &dyn std::any::Any;
	fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
	fn setup_within_container(&mut self, _container: &mut UiElementContainerData) {}

	fn parent_size_changed(
		&mut self,
		_container_data: &mut UiElementContainerData,
		_parent_size: &Vector2,
	) {
	}
	fn recalculate_size(&mut self, container: &mut UiElementContainerData) {
		let mut r = Rectangle::default()
			.with_center(&Vector2::zero())
			.with_size(&container.size);

		for c in container.borrow_children().iter() {
			let rc = Rectangle::default()
				.with_center(c.borrow().pos())
				.with_size(c.borrow().size());
			r = r.combine_with(&rc);
		}

		container.set_size(r.size());
	}
	fn add_child(&mut self, _child: &mut UiElementContainerData) {}
	fn update(&mut self, _container: &mut UiElementContainerData, _time_step: f64) {}
	fn render(&self, _container: &UiElementContainerData, _ui_renderer: &mut UiRenderer) {}
	fn layout(&mut self, container: &mut UiElementContainerData, _pos: &Vector2) {
		for c in container.borrow_children_mut().iter_mut() {
			c.borrow_mut().layout(&Vector2::zero());
		}
		self.recalculate_size(container);
	}
	fn render_debug(
		&self,
		_container: &UiElementContainerData,
		_debug_renderer: &mut DebugRenderer,
		_offset: &Vector2,
	) {
	}
	fn handle_ui_event(
		&mut self,
		_container: &mut UiElementContainerData,
		_event: &UiEvent,
		_event_sender: &Sender<Box<dyn UiEventResponse>>,
	) -> Option<Box<dyn UiEventResponse>> {
		//		Vec::new()
		None
	}
	fn handle_ui_event_response(
		&mut self,
		_container_data: &mut UiElementContainerData,
		response: Box<dyn UiEventResponse>,
	) -> Option<Box<dyn UiEventResponse>> {
		Some(response)
	}
	fn preferred_size(&self) -> Option<&Vector2> {
		None
	}
	fn configure_from_yaml(&mut self, yaml: &str) {
		let value: serde_yaml::Value = serde_yaml::from_str(&yaml).unwrap();
		self.configure_from_yaml_value(value)
	}

	fn configure_from_yaml_value(&mut self, _yaml_value: serde_yaml::Value) {
		panic!(
			"configure_from_yaml_value not implemented for {}",
			self.type_name()
		);
	}

	fn to_yaml_config(&self) -> serde_yaml::Value {
		serde_yaml::Value::Null
	}

	fn containerize(self) -> UiElementContainer
	where
		Self: 'static + Sized,
	{
		UiElementContainer::new(Box::new(self))
	}
	fn dump_info(&self) {}
}

impl std::fmt::Debug for dyn UiElement {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
		write!(f, "{}", self.type_name())
	}
}

pub struct UiElementInfo {
	pub type_name:   &'static str,
	pub producer_fn: &'static dyn Fn() -> Box<dyn UiElement>,
}
