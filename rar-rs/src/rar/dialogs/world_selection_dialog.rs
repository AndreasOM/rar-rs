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
		let container = UiGravityBox::new()
			.containerize()
			.with_size(size)
			.with_child_element_containers(
				[
					{
						UiButton::new("ui-button", size)
							.containerize()
							.with_name(name)
							.with_fade_out(0.0)
							.with_fade_in(1.0)
					},
					{
						UiLabel::new(&size, name)
							.containerize()
							.with_name(name)
							.with_fade_out(0.0)
							.with_fade_in(1.0)
					},
				]
				.into(),
			);
		container
	}
}

impl UiElement for WorldSelectionDialog {
	fn type_name(&self) -> &str {
		"[WorldSelectionDialog]"
	}
	fn as_any(&self) -> &dyn std::any::Any {
		self
	}
	fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
		self
	}

	fn setup_within_container(&mut self, container: &mut UiElementContainerData) {
		let button_size = Vector2::new(256.0, 64.0);
		container.add_child_element_container(
			UiGridBox::default()
				.with_padding(16.0)
				.with_column_count(1)
				.containerize()
				.with_name("World Selection Dialog - vbox")
				.with_child_element_containers(
					[
						Self::create_world_button("dev", &button_size),
						Self::create_world_button("debug", &button_size),
						Self::create_world_button("grassland", &button_size),
						Self::create_world_button("mystic_mountain", &button_size),
						Self::create_world_button("DebugCollisions", &button_size),
						Self::create_world_button("Settings", &button_size),
						Self::create_world_button("Quit", &button_size),
					]
					.into(),
				),
		);
	}
}
