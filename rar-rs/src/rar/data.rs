use std::sync::RwLock;

#[derive(Debug, Default)]
pub struct AudioData {
	pub is_sound_enabled: bool,
	pub is_music_enabled: bool,
}

#[derive(Debug)]
pub struct RarData {
	pub audio: RwLock<AudioData>,
}

impl RarData {
	pub fn new() -> Self {
		Self {
			audio: RwLock::new(AudioData::default()),
		}
	}
}

impl oml_game::system::Data for RarData {
	fn as_any(&self) -> &dyn std::any::Any {
		self
	}
	fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
		self
	}
}
