use oml_game::math::Vector2;
use oml_game::renderer::Color;
use serde::Deserialize;

use crate::ui::UiElementInfo;
use crate::ui::{UiElement, UiElementContainerData, UiElementFadeState, UiRenderer};

#[derive(Default)]
pub struct UiSpacer {
	size:  Vector2,
	color: Color,
}

impl UiSpacer {
	pub fn new(size: &Vector2, color: &Color) -> Self {
		Self {
			size:  *size,
			color: *color,
		}
	}

	pub fn from_yaml(yaml: &str) -> Self {
		let config: UiSpacerConfig = serde_yaml::from_str(&yaml).unwrap();

		Self {
			size:  Vector2::from_x_str(&config.size),
			color: Color::white(),
		}
	}
	pub fn info() -> &'static UiElementInfo {
		&UiElementInfo {
			type_name:   "UiSpacer",
			producer_fn: &Self::produce,
		}
	}

	pub fn produce() -> Box<dyn UiElement> {
		Box::new(Self::default())
	}
}

static UI_LABEL_VISIBLE: bool = false;

impl UiElement for UiSpacer {
	fn type_name(&self) -> &str {
		"[UiSpacer]"
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
		if UI_LABEL_VISIBLE {
			if *container.fade_state() != UiElementFadeState::FadedOut {
				let l = container.get_fade_level();
				ui_renderer.push_color(&self.color);
				ui_renderer.push_opacity(l);
				ui_renderer.render_quad(&container.pos, &self.size);
				ui_renderer.pop_opacity();
				ui_renderer.pop_color();
			}
		}
	}
	fn configure_from_yaml_value(&mut self, yaml_value: serde_yaml::Value) {
		let config: UiSpacerConfig = serde_yaml::from_value(yaml_value).unwrap();

		self.size = Vector2::from_x_str(&config.size);
		self.color = Color::default(); // :TODO:
	}
	/*
	fn configure_from_yaml(&mut self, yaml: &str) {
		let config: UiSpacerConfig = serde_yaml::from_str(&yaml).unwrap();
		self.size = Vector2::from_x_str(&config.size);
		self.color = Color::default(); // :TODO:
	}
	*/
}

#[derive(Debug, Deserialize)]
struct UiSpacerConfig {
	size: String,
}
