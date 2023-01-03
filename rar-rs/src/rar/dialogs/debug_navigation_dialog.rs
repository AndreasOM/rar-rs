use oml_game::math::Vector2;

use crate::ui::*;

#[derive(Debug, Default)]
pub struct DebugNavigationDialog {}

impl DebugNavigationDialog {
	pub fn new() -> Self {
		Default::default()
	}
}

impl UiElement for DebugNavigationDialog {
	fn as_any(&self) -> &dyn std::any::Any {
		self
	}
	fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
		self
	}

	fn setup_within_container(&mut self, container: &mut UiElementContainerData) {
		//let button_size = Vector2::new(256.0, 64.0);
		container.add_child_element_container(
			UiHbox::new()
				.with_padding(16.0)
				.containerize()
				.with_name("Debug Navigation Dialog - hbox")
				.with_child_element_containers(
					[
						//Self::create_button("back", &button_size)
						{
							UiButton::new("ui-button_back", &Vector2::new(64.0, 64.0))
								.containerize()
								.with_name("back")
								.with_fade_out(0.0)
								.with_fade_in(1.0)
						},
					]
					.into(),
				),
		);
	}
}
