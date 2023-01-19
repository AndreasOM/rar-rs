use oml_game::math::Vector2;
use oml_game::system::System;
use tracing::*;

use crate::ui::*;

#[derive(Debug, Default)]
pub struct WorldSelectionDialog {
	container: Option<UiElementContainer>,
}

impl WorldSelectionDialog {
	fn create_world_button(
		system: &mut System,
		ui_element_factory: &UiElementFactory,
	) -> UiElementContainer {
		if let Some(world_button) =
			UiElementContainer::from_config_asset(system, ui_element_factory, "world_button")
		{
			world_button
		} else {
			todo!();
		}
	}
	pub fn new(system: &mut System, ui_element_factory: &UiElementFactory) -> Self {
		let mut container = UiElementContainer::from_config_asset(
			system,
			ui_element_factory,
			"world_selection_dialog",
		);
		if let Some(container) = &mut container {
			if !container.find_child_container_by_tag_mut_then("world_selection_box", &mut |wsb| {
				for _i in 0..3 {
					wsb.add_child_element_container(Self::create_world_button(
						system,
						ui_element_factory,
					));
				}
			}) {
				warn!("world_selection_box not found");
				todo!();
			}
			container.recalculate_size();
		}

		Self { container }
	}
	/*
	fn create_world_button(name: &str, size: &Vector2) -> UiElementContainer {
		let container = UiButton::new("ui-button", size)
			.containerize()
			.with_name(name)
			.with_fade_out(0.0)
			.with_fade_in(1.0)
			.with_child_element_containers(
				[{
					UiLabel::new(&size, name)
						.containerize()
						.with_name(name)
						.with_fade_out(0.0)
						.with_fade_in(1.0)
				}]
				.into(),
			);
		container
	}
	*/
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
	fn setup_within_container(&mut self, container_data: &mut UiElementContainerData) {
		if let Some(container) = self.container.take() {
			container_data.add_child_element_container(container);
		} else {
			panic!("No container for IngamePauseDialog");
		};
	}
}
