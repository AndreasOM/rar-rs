use std::sync::RwLock;

#[derive(Debug, Default)]
pub struct AudioData {
	pub is_sound_enabled: bool,
	pub is_music_enabled: bool,
}

#[derive(Debug, Default)]
pub struct GameData {
	pub is_game_paused: bool,
}

#[derive(Debug)]
pub struct RarData {
	pub audio: RwLock<AudioData>,
	pub game:  RwLock<GameData>,
}

impl RarData {
	pub fn new() -> Self {
		Self {
			audio: RwLock::new(AudioData::default()),
			game:  RwLock::new(GameData::default()),
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
