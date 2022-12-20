use std::sync::Arc;

use oml_game::math::Vector2;
use oml_game::system::Data;
use oml_game::system::System;
use tracing::*;

use crate::rar::data::RarData;
use crate::ui::*;

#[derive(Debug)]
pub struct SettingsDialog {
	base_data_build_number: String,
	data:                   Arc<dyn Data>,
}

impl SettingsDialog {
	pub fn new(system: &mut System) -> Self {
		let base_data_build_number_path = "base_build_number.txt";
		let mut f = system
			.default_filesystem_mut()
			.open(base_data_build_number_path);
		let base_data_build_number = if f.is_valid() {
			f.read_as_string()
		} else {
			"[file not found]".to_string()
		};
		Self {
			base_data_build_number,
			data: Arc::clone(system.data()),
			//..Default::default()
		}
	}
	fn create_children(&self) -> Vec<UiElementContainer> {
		let label_size = Vector2::new(768.0, 96.0);
		const VERSION: &str = env!("CARGO_PKG_VERSION");
		const BUILD_DATETIME: &str = env!("BUILD_DATETIME");
		const GIT_COMMIT_HASH: &str = env!("GIT_COMMIT_HASH");

		let code_build_number = env!("CODE_BUILD_NUMBER");
		[UiHbox::new()
			.with_padding(16.0)
			.containerize()
			.with_name("Settings hBox")
			.with_child_element_containers(
				[
					{
						UiButton::new("ui-button_back", &Vector2::new(64.0, 64.0))
							.containerize()
							.with_name("back")
							.with_fade_out(0.0)
							.with_fade_in(1.0)
					},
					{
						UiVbox::new()
							.with_padding(16.0)
							.containerize()
							.with_name("Settings hBox")
							.with_child_element_containers(
								[
									{
										UiToggleButton::new(
											"ui-togglebutton_music_on",
											"ui-togglebutton_music_off",
											&Vector2::new(64.0, 64.0),
										)
										.containerize()
										.with_name("music/toggle")
										.with_fade_out(0.0)
										.with_fade_in(1.0)
									},
									{
										UiToggleButton::new(
											"ui-togglebutton_sound_on",
											"ui-togglebutton_sound_off",
											&Vector2::new(64.0, 64.0),
										)
										.containerize()
										.with_name("sound/toggle")
										.with_fade_out(0.0)
										.with_fade_in(1.0)
									},
								]
								.into(),
							)
					},
					{
						UiVbox::new()
							.with_padding(16.0)
							.containerize()
							.with_name("Settings hBox")
							.with_child_element_containers(
								[
									{
										UiLabel::new(&label_size, &format!("Version : {}", VERSION))
											.containerize()
									},
									{
										UiLabel::new(
											&label_size,
											&format!("Build at: {}", BUILD_DATETIME),
										)
										.containerize()
									},
									{
										UiLabel::new(
											&label_size,
											&format!("Code Build#: {}", code_build_number),
										)
										.containerize()
									},
									{
										UiLabel::new(
											&label_size,
											&format!(
												"'base' data Build#: {}",
												self.base_data_build_number
											),
										)
										.containerize()
									},
									{
										UiLabel::new(
											&label_size,
											&format!("Commit : {}", GIT_COMMIT_HASH),
										)
										.containerize()
									},
								]
								.into(),
							)
					},
				]
				.into(),
			)]
		.into()
	}

	fn update_music(
		&self,
		uielement: &dyn UiElement,
		container: &mut UiElementContainerData,
		is_on: bool,
	) {
		if let Some(mut stb) = container.find_child_mut(&[
			"Settings Dialog - vbox",
			"Settings hBox",
			"Settings hBox",
			"music/toggle",
		]) {
			// debug!("Found sound/toggle");
			let mut stb = stb.borrow_mut();
			let stb = stb.borrow_element_mut();
			match stb.as_any_mut().downcast_mut::<UiToggleButton>() {
				Some(stb) => {
					if is_on {
						stb.goto_a();
					} else {
						stb.goto_b();
					}
				},
				None => panic!("{:?} isn't a UiToggleButton!", &stb),
			};
		} else {
			uielement.dump_info();
			todo!("Fix path to music toggle button");
		}
	}

	fn update_sound(
		&self,
		uielement: &dyn UiElement,
		container: &mut UiElementContainerData,
		is_on: bool,
	) {
		if let Some(mut stb) = container.find_child_mut(&[
			"Settings Dialog - vbox",
			"Settings hBox",
			"Settings hBox",
			"sound/toggle",
		]) {
			// debug!("Found sound/toggle");
			let mut stb = stb.borrow_mut();
			let stb = stb.borrow_element_mut();
			match stb.as_any_mut().downcast_mut::<UiToggleButton>() {
				Some(stb) => {
					if is_on {
						stb.goto_a();
					} else {
						stb.goto_b();
					}
				},
				None => panic!("{:?} isn't a UiToggleButton!", &stb),
			};
		} else {
			uielement.dump_info();
			todo!("Fix path to sound toggle button");
		}
	}
}

impl UiElement for SettingsDialog {
	fn as_any(&self) -> &dyn std::any::Any {
		self
	}
	fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
		self
	}

	fn setup_within_container(&mut self, container: &mut UiElementContainerData) {
		//let button_size = Vector2::new(256.0, 64.0);
		container.add_child_element_container(
			UiVbox::new()
				.with_padding(16.0)
				.containerize()
				.with_name("Settings Dialog - vbox")
				.with_child_element_containers(self.create_children().into()),
		);
	}
	fn update(&mut self, container: &mut UiElementContainerData, _time_step: f64) {
		match self.data.as_any().downcast_ref::<RarData>() {
			Some(data) => {
				data.audio.read().and_then(|audio| {
					// :TODO: maybe use try_read instead of potentially blocking
					//debug!("is_sound_enabled {:?}", audio.is_sound_enabled);
					//debug!("is_music_enabled {:?}", audio.is_music_enabled);
					let uielement: &dyn UiElement = self;
					self.update_sound(uielement, container, audio.is_sound_enabled);
					self.update_music(uielement, container, audio.is_music_enabled);
					Ok(())
				});
			},
			None => {},
		}
	}
}
