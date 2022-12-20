use oml_game::math::Vector2;
use oml_game::system::System;

use crate::ui::*;

#[derive(Debug, Default)]
pub struct SettingsDialog {
	base_data_build_number: String,
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
			..Default::default()
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
	fn update(&mut self, _container: &UiElementContainerData, _time_step: f64) {}
}
