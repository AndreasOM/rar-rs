use oml_game::system::System;
use tracing::*;

use crate::rar::WorldList;
use crate::ui::*;

#[derive(Debug, Default)]
pub struct WorldSelectionDialog {
	container: Option<UiElementContainer>,
}

impl WorldSelectionDialog {
	fn create_world_button(
		system: &mut System,
		ui_element_factory: &UiElementFactory,
		id: &str,
		name: &str,
	) -> UiElementContainer {
		if let Some(mut world_button) =
			UiElementContainer::from_config_asset(system, ui_element_factory, "world_button")
		{
			world_button.find_child_by_tag_as_mut_element_then::<UiLabel>(
				//.
				"label",
				&|l| {
					//.
					l.set_text(name);
				},
			); /* :TODO: {
	   warn!("'label' not found for world_button");
   }
   */
			world_button.with_name(id)
		// world_button
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
				let world_list = WorldList::from_config_asset(system, "worlds").unwrap();
				debug!("{:?}", &world_list);
				for w in world_list.worlds().iter() {
					wsb.add_child_element_container(Self::create_world_button(
						system,
						ui_element_factory,
						w.id(),
						w.name(),
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
