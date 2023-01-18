use oml_game::math::Vector2;
use oml_game::renderer::Color;
use serde::Deserialize;

use crate::ui::UiElementInfo;
use crate::ui::{UiElement, UiElementContainerData, UiElementFadeState, UiRenderer};

#[derive(Debug, Default)]
pub struct UiLabel {
	size:      Vector2,
	color:     Color,
	text:      String,
	alignment: Vector2,
	font_id:   u8,
}

impl UiLabel {
	pub fn new(size: &Vector2, text: &str) -> Self {
		Self {
			size:      *size,
			color:     Color::from_rgba(0.8, 0.8, 0.8, 0.8),
			text:      text.to_owned(),
			alignment: Vector2::new(-1.0, 0.0),
			font_id:   0,
		}
	}

	pub fn set_alignment(&mut self, alignment: &Vector2) {
		self.alignment = *alignment;
	}

	pub fn set_text(&mut self, text: &str) {
		self.text = text.to_owned();
	}

	pub fn set_color(&mut self, color: &Color) {
		self.color = *color;
	}

	pub fn set_font_id(&mut self, font_id: u8) {
		self.font_id = font_id;
	}

	pub fn with_font_id(mut self, font_id: u8) -> Self {
		self.font_id = font_id;
		self
	}
	pub fn info() -> &'static UiElementInfo {
		&UiElementInfo {
			type_name:   "UiLabel",
			producer_fn: &Self::produce,
		}
	}

	pub fn produce() -> Box<dyn UiElement> {
		Box::new(Self::default())
	}
}

impl UiElement for UiLabel {
	fn type_name(&self) -> &str {
		Self::info().type_name
	}
	fn as_any(&self) -> &dyn std::any::Any {
		self
	}
	fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
		self
	}
	fn preferred_size(&self) -> Option<&Vector2> {
		Some(&self.size)
	}
	fn render(&self, container: &UiElementContainerData, ui_renderer: &mut UiRenderer) {
		if *container.fade_state() != UiElementFadeState::FadedOut {
			let l = container.get_fade_level();
			ui_renderer.push_color(&self.color);
			ui_renderer.push_opacity(l);
			ui_renderer.push_font_id(self.font_id);
			ui_renderer.print(&container.pos, &container.size, &self.alignment, &self.text);

			ui_renderer.pop_font_id();
			ui_renderer.pop_opacity();
			ui_renderer.pop_color();
		}
	}
	fn configure_from_yaml_value(&mut self, yaml_value: serde_yaml::Value) {
		let config: UiLabelConfig = serde_yaml::from_value(yaml_value).unwrap();

		self.size = Vector2::from_x_str(&config.size);
		self.text = config.text;
		if let Some(_color) = config.color {
			// :TODO:
		}
	}
}

#[derive(Debug, Deserialize)]
struct UiLabelConfig {
	size:  String,
	#[serde(default)]
	text:  String,
	color: Option<String>,
	//alignment: Vector2, // :TODO:
}
