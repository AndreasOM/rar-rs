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
}

impl UiElement for WorldSelectionDialog {
	fn as_any(&self) -> &dyn std::any::Any {
		self
	}
	fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
		self
	}
	fn setup_within_container(&mut self, container: &mut UiElementContainerData) {
		let mut vbox = UiVbox::new();
		vbox.set_padding(16.0);

		//		let mut vbox = container.add_child_element(vbox);
		let mut vbox = vbox.containerize();
/*
		let mut vbox = container.add_child_element_container(vbox);
		let mut vbox = vbox.borrow_mut();
*/
		vbox.set_name("World Selection Dialog - vbox");

		/*
		let world_dev_button =
			vbox.add_child_element(UiButton::new("ui-button", &Vector2::new(256.0, 64.0)));
		{
			let mut world_dev_button = world_dev_button.borrow_mut();
			world_dev_button.set_name("dev");
			world_dev_button.fade_out(0.0);
			world_dev_button.fade_in(1.0);
		}
		*/
		/*
		let world_dev_button_container = {
			let world_dev_button = UiButton::new("ui-button", &Vector2::new(256.0, 64.0));
			let mut world_dev_button_container = UiElementContainer::new(Box::new(world_dev_button));

			world_dev_button_container.set_name("dev");
			world_dev_button_container.fade_out(0.0);
			world_dev_button_container.fade_in(1.0);

			world_dev_button_container
		};
		vbox.add_child_element_container( world_dev_button_container );
		*/
		vbox.add_child_element_container({
			let world_dev_button = UiButton::new("ui-button", &Vector2::new(256.0, 64.0));
			//let mut world_dev_button_container = UiElementContainer::new(Box::new(world_dev_button));
			//let mut world_dev_button_container = UiButton::containerize( world_dev_button );
			let mut world_dev_button_container = world_dev_button.containerize();

			world_dev_button_container.set_name("dev");
			world_dev_button_container.fade_out(0.0);
			world_dev_button_container.fade_in(1.0);

			world_dev_button_container
		});

		vbox.add_child_element_container({
			let world_debug_button = UiButton::new("ui-button", &Vector2::new(256.0, 64.0));
			let mut world_debug_button_container = world_debug_button.containerize();

			world_debug_button_container.set_name("debug");
			world_debug_button_container.fade_out(0.0);
			world_debug_button_container.fade_in(1.0);

			world_debug_button_container
		});

		container.add_child_element_container(vbox);
	}
}
