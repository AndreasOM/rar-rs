use oml_game::math::Vector2;
use oml_game::renderer::debug_renderer::DebugRenderer;
use serde::{Deserialize, Serialize};
use tracing::*;

use crate::ui::UiElementInfo;
use crate::ui::{UiElement, UiElementContainerData, UiElementFadeState, UiRenderer};

#[derive(Debug, Default)]
pub struct UiImage {
	imagename: String,
	imagesize: Vector2,
}

impl UiImage {
	pub fn new(imagename: &str, size: &Vector2) -> Self {
		Self {
			imagename: imagename.to_owned(),
			imagesize: *size,
		}
	}
	pub fn info() -> &'static UiElementInfo {
		&UiElementInfo {
			type_name:   "UiImage",
			producer_fn: &Self::produce,
		}
	}

	pub fn produce() -> Box<dyn UiElement> {
		Box::new(Self::default())
	}
}

impl UiElement for UiImage {
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
		Some(&self.imagesize)
	}
	fn render(&self, container: &UiElementContainerData, ui_renderer: &mut UiRenderer) {
		if *container.fade_state() != UiElementFadeState::FadedOut {
			let l = container.get_fade_level();
			ui_renderer.push_opacity(l);
			ui_renderer.use_texture(&self.imagename);
			ui_renderer.render_textured_quad(&container.pos, &self.imagesize);
			ui_renderer.pop_opacity();
		}
	}
	fn configure_from_yaml_value(&mut self, yaml_value: serde_yaml::Value) {
		let config: UiImageConfig = serde_yaml::from_value(yaml_value).unwrap();

		self.imagesize = Vector2::from_x_str(&config.size);
		self.imagename = config.image;
	}
	fn to_yaml_config(&self) -> serde_yaml::Value {
		serde_yaml::to_value(UiImageConfig {
			image: self.imagename.clone(),
			size:  format!("{}x{}", self.imagesize.x, self.imagesize.y),
		})
		.unwrap_or(serde_yaml::Value::Null)
	}
}

#[derive(Debug, Deserialize, Serialize)]
struct UiImageConfig {
	image: String,
	size:  String,
}
