use oml_game::math::Vector2;

use crate::ui::*;

#[derive(Debug, Default)]
pub struct WorldSelectionDialog {}

impl WorldSelectionDialog {
	pub fn new() -> Self {
		Self {
			..Default::default()
		}
	}
	fn create_world_button(name: &str, size: &Vector2) -> UiElementContainer {
		let mut world_button_container = UiButton::new("ui-button", size)
			.containerize()
			.with_name(name);

		world_button_container.fade_out(0.0);
		world_button_container.fade_in(1.0);

		world_button_container
	}
}

impl UiElement for WorldSelectionDialog {
	fn as_any(&self) -> &dyn std::any::Any {
		self
	}
	fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
		self
	}

	fn setup_within_container(&mut self, container: &mut UiElementContainerData) {
		let button_size = Vector2::new(256.0, 64.0);
		container.add_child_element_container(
			UiVbox::new()
				.with_padding(16.0)
				.containerize()
				.with_name("World Selection Dialog - vbox")
				.with_child_element_containers(
					[
						Self::create_world_button("dev", &button_size),
						Self::create_world_button("debug", &button_size),
					]
					.into(),
				),
		);
	}
}
