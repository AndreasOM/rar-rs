use oml_game::math::Vector2;
use oml_game::window::window_update_context::WindowUpdateContext;
use tracing::*;

use crate::rar::AudioMessage;
use crate::ui::UiUpdateContext;

#[derive(Debug)]
pub struct AppUpdateContext {
	time_step:         f64,
	cursor_pos:        Vector2,
	wuc:               Option<WindowUpdateContext>,
	sound_tx:          Option<std::sync::mpsc::Sender<AudioMessage>>,
	is_music_playing:  bool,
	is_sound_enabled:  bool,
	ui_update_context: Option<Box<dyn UiUpdateContext>>,
}

impl AppUpdateContext {
	pub fn new() -> Self {
		Self {
			time_step:         0.0,
			cursor_pos:        Vector2::zero(),
			wuc:               None,
			sound_tx:          None,
			is_music_playing:  false,
			is_sound_enabled:  true,
			ui_update_context: None,
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
	pub fn with_is_sound_enabled(mut self, is_sound_enabled: bool) -> Self {
		self.is_sound_enabled = is_sound_enabled;
		self
	}

	pub fn is_sound_enabled(&self) -> bool {
		self.is_sound_enabled
	}

	pub fn with_ui_update_context(mut self, ui_update_context: Box<dyn UiUpdateContext>) -> Self {
		self.ui_update_context = Some(ui_update_context);
		self
	}

	pub fn ui_update_context_as_then<E: 'static>(&mut self, f: &dyn Fn(&mut E)) {
		if let Some(uuc) = &mut self.ui_update_context {
			match uuc.as_any_mut().downcast_mut::<E>() {
				Some(e) => {
					f(e);
				},
				None => panic!("{:?} isn't a {:?}!", &uuc, std::any::type_name::<E>(),),
			}
		} else {
			warn!("No UiUpdateContext!",);
		}
	}
}
