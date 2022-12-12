use oml_game::math::Vector2;
use oml_game::window::window_update_context::WindowUpdateContext;

use crate::rar::AudioMessage;

#[derive(Debug, Clone)]
pub struct AppUpdateContext {
	time_step:        f64,
	cursor_pos:       Vector2,
	wuc:              Option<WindowUpdateContext>,
	sound_tx:         Option<std::sync::mpsc::Sender<AudioMessage>>,
	is_music_playing: bool,
}

impl AppUpdateContext {
	pub fn new() -> Self {
		Self {
			time_step:        0.0,
			cursor_pos:       Vector2::zero(),
			wuc:              None,
			sound_tx:         None,
			is_music_playing: false,
		}
	}

	pub fn time_step(&self) -> f64 {
		self.time_step
	}

	pub fn set_time_step(mut self, time_step: f64) -> Self {
		self.time_step = time_step;
		self
	}

	pub fn cursor_pos(&self) -> &Vector2 {
		&self.cursor_pos
	}

	pub fn set_cursor_pos(mut self, cursor_pos: &Vector2) -> Self {
		self.cursor_pos = *cursor_pos;
		self
	}

	pub fn wuc(&self) -> Option<WindowUpdateContext> {
		self.wuc
	}

	pub fn set_wuc(mut self, wuc: &WindowUpdateContext) -> Self {
		self.wuc = Some(*wuc);
		self
	}

	pub fn sound_tx(&mut self) -> &mut Option<std::sync::mpsc::Sender<AudioMessage>> {
		&mut self.sound_tx
	}

	pub fn set_sound_tx(mut self, sound_tx: std::sync::mpsc::Sender<AudioMessage>) -> Self {
		self.sound_tx = Some(sound_tx);
		self
	}

	pub fn with_is_music_playing(mut self, is_music_playing: bool) -> Self {
		self.is_music_playing = is_music_playing;
		self
	}

	pub fn is_music_playing(&self) -> bool {
		self.is_music_playing
	}
}
