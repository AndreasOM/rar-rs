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
	data:                   Option<Arc<dyn Data>>,
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
			data: system.data().as_ref().map(|data| Arc::clone(data)),
			//..Default::default()
		}
	}
	fn create_info_labels(&self) -> Vec<UiElementContainer> {
		let label_size = Vector2::new(256.0 + 64.0 + 32.0, 96.0);
		let value_size = Vector2::new(512.0, 96.0);
		const VERSION: &str = env!("CARGO_PKG_VERSION");
		const BUILD_DATETIME: &str = env!("BUILD_DATETIME");
		const GIT_COMMIT_HASH: &str = env!("GIT_COMMIT_HASH");
		let code_build_number = env!("CODE_BUILD_NUMBER");

		let labels = [
			("Version           :", VERSION),
			("Build at          :", BUILD_DATETIME),
			("Code Build#       :", &format!("{}", code_build_number)),
			(
				"'base' data Build#:",
				&format!("{}", self.base_data_build_number),
			),
			("Commit            :", GIT_COMMIT_HASH),
		];
		let mut vl = Vec::new();
		for l in labels {
			vl.push(UiLabel::new(&label_size, l.0).containerize());
		}
		let mut vv = Vec::new();
		for l in labels {
			vv.push(UiLabel::new(&value_size, l.1).containerize());
		}
		// Note: We could have done hbox in vbox instead. Should really have a gridbox ;)
		[
			UiVbox::new()
				.with_padding(16.0)
				.containerize()
				.with_name("Labels")
				.with_child_element_containers(vl),
			UiVbox::new()
				.with_padding(16.0)
				.containerize()
				.with_name("Values")
				.with_child_element_containers(vv),
		]
		.into()
	}
	fn create_audio_buttons(&self) -> Vec<UiElementContainer> {
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
		.into()
	}
	fn create_children(&self) -> Vec<UiElementContainer> {
		[UiHbox::new()
			.with_padding(16.0)
			.containerize()
			.with_name("Settings hBox")
			.with_child_element_containers(
				[
					{
						UiVbox::new()
							.with_padding(16.0)
							.containerize()
							.with_name("Settings vBox") // :TODO: fix name
							.with_child_element_containers(self.create_audio_buttons())
					},
					{
						UiHbox::new()
							.with_padding(16.0)
							.containerize()
							.with_name("Labels hBox")
							.with_child_element_containers(self.create_info_labels())
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
		container.find_child_mut_as_element_then::<UiToggleButton>(
			&[
				"Settings Dialog - vbox",
				"Settings hBox",
				"Settings hBox",
				"music/toggle",
			],
			&|stb| {
				// debug!("Found music/toggle");
				if is_on {
					stb.goto_a();
				} else {
					stb.goto_b();
				}
			},
		);
	}

	fn update_sound(
		&self,
		uielement: &dyn UiElement,
		container: &mut UiElementContainerData,
		is_on: bool,
	) {
		container.find_child_mut_as_element_then::<UiToggleButton>(
			&[
				"Settings Dialog - vbox",
				"Settings hBox",
				"Settings hBox",
				"sound/toggle",
			],
			&|stb| {
				// debug!("Found sound/toggle");
				if is_on {
					stb.goto_a();
				} else {
					stb.goto_b();
				}
			},
		);
	}
}

impl UiElement for SettingsDialog {
	fn type_name(&self) -> &str {
		"[SettingsDialog]"
	}
	fn as_any(&self) -> &dyn std::any::Any {
		self
	}
	fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
		self
	}

	fn setup_within_container(&mut self, container: &mut UiElementContainerData) {
		//let button_size = Vector2::new(256.0, 64.0);
		container.add_child_element_container(
			Ui3x3Image::new(
				"ui-3x3-grassland",
				&Vector2::new(192.0 * 6.0, 192.0 * 3.0),
				&Vector2::new(192.0, 192.0),
			)
			.containerize()
			.with_name("Settings Dialog - background"),
		);
		container.add_child_element_container(
			UiVbox::new()
				.with_padding(16.0)
				.containerize()
				.with_name("Settings Dialog - vbox")
				.with_child_element_containers(self.create_children().into()),
		);
	}
	fn update(&mut self, container: &mut UiElementContainerData, _time_step: f64) {
		if let Some(data) = &self.data {
			match data.as_any().downcast_ref::<RarData>() {
				Some(data) => {
					data.audio.read().and_then(|audio| {
						// :TODO: maybe use try_read instead of potentially blocking
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
}
