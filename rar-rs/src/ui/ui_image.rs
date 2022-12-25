use oml_game::math::Vector2;
use oml_game::renderer::debug_renderer::DebugRenderer;
use tracing::*;

use crate::ui::{UiElement, UiElementContainerData, UiElementFadeState, UiRenderer};

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
}

impl UiElement for UiImage {
	fn type_name(&self) -> &str {
		"[UiImage]"
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
	/*
	fn update( &mut self, _time_step: f64 ) {
	}
	*/
	fn render(&self, container: &UiElementContainerData, ui_renderer: &mut UiRenderer) {
		if *container.fade_state() != UiElementFadeState::FadedOut {
			let l = container.get_fade_level();
			ui_renderer.push_opacity(l);
			ui_renderer.use_texture(&self.imagename);
			ui_renderer.render_textured_quad(&container.pos, &self.imagesize);
			//			ui_renderer.render_textured_quad(&Vector2::new(-900.0, 500.0), &self.imagesize);
			ui_renderer.pop_opacity();
		}
	}
	fn render_debug(
		&self,
		_container: &UiElementContainerData,
		_debug_renderer: &mut DebugRenderer,
		offset: &Vector2,
	) {
		debug!("{}, {}", offset.x, offset.y);
	}
}
