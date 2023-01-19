use std::sync::Arc;

use oml_game::math::Vector2;
use oml_game::system::Data;
use oml_game::system::System;
use tracing::*;

use crate::rar::data::RarData;
use crate::rar::font_ids::FontId;
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
		let h = 16.0;
		let label_size = Vector2::new(256.0 + 64.0 + 32.0, h);
		let value_size = Vector2::new(512.0, h);
		const VERSION: &str = env!("CARGO_PKG_VERSION");
		const BUILD_DATETIME: &str = env!("BUILD_DATETIME");
		const GIT_COMMIT_HASH: &str = env!("GIT_COMMIT_HASH");
		let code_build_number = env!("CODE_BUILD_NUMBER");

		let labels = [
			"Version",
			&format!(": {}", VERSION),
			"Build at",
			&format!(": {}", BUILD_DATETIME),
			"Code Build#",
			&format!(": {}", code_build_number),
			"'base' data Build#",
			&format!(": {}", self.base_data_build_number),
			"Commit",
			&format!(": {}", GIT_COMMIT_HASH),
		];

		labels
			//.iter()
			.chunks_exact(2)
			.flat_map(|labels| {
				[
					UiLabel::new(&label_size, labels[0])
						.with_font_id(FontId::Mono as u8)
						.containerize(),
					UiLabel::new(&value_size, labels[1])
						.with_font_id(FontId::Mono as u8)
						.containerize(),
				]
			})
			.collect()
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
				.with_tag("music/toggle")
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
				.with_tag("sound/toggle")
				.with_fade_out(0.0)
				.with_fade_in(1.0)
			},
		]
		.into()
	}
	fn create_children(&self) -> Vec<UiElementContainer> {
		[
			//UiHbox::new()
			UiGridBox::default()
				.with_column_count(9)
				.with_padding(16.0)
				.containerize()
				.with_name("Settings hBox")
				.with_child_element_containers(
					[
						{
							// UiVbox::new()
							UiGridBox::default()
								.with_column_count(1)
								.with_padding(16.0)
								.containerize()
								.with_name("Settings vBox") // :TODO: fix name
								.with_child_element_containers(self.create_audio_buttons())
						},
						{
							UiGridBox::default()
								.with_padding(16.0)
								.with_column_count(2)
								.containerize()
								.with_name("Labels hBox")
								.with_child_element_containers(self.create_info_labels())
						},
					]
					.into(),
				),
		]
		.into()
	}

	fn update_music(
		&self,
		_uielement: &dyn UiElement,
		container_data: &mut UiElementContainerData,
		is_on: bool,
	) {
		container_data.find_child_by_tag_as_mut_element_then::<UiToggleButton>(
			"music/toggle",
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
		_uielement: &dyn UiElement,
		container_data: &mut UiElementContainerData,
		is_on: bool,
	) {
		container_data.find_child_by_tag_as_mut_element_then::<UiToggleButton>(
			"sound/toggle",
			&|stb| {
				debug!("Found sound/toggle");
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
			//UiVbox::new()
			UiGridBox::default()
				.with_column_count(1)
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
