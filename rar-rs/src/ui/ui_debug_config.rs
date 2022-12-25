use std::collections::HashMap;
use std::sync::Arc;
use std::sync::RwLock;

use once_cell::sync::Lazy;

#[derive(Debug, Copy, Clone, Default)]
pub enum UiDebugConfigMode {
	All,
	Selected,
	#[default]
	None,
}

#[derive(Debug, Default)]
pub struct UiDebugConfig {
	mode:     UiDebugConfigMode,
	selected: HashMap<String, usize>,
}

impl UiDebugConfig {
	pub fn set_mode(&mut self, mode: UiDebugConfigMode) {
		self.mode = mode;
	}
	pub fn mode(&self) -> UiDebugConfigMode {
		self.mode
	}
	pub fn select(&mut self, name: &str, depth: usize) {
		let selected = self.selected.entry(name.to_string()).or_insert(0);
		*selected = depth;
	}

	pub fn is_selected(&self, name: &str) -> Option<usize> {
		self.selected.get(name).copied()
	}

	pub fn write_then(f: &mut dyn for<'a> FnMut(&'a mut UiDebugConfig)) {
		UIDEBUGCONFIG
			.write()
			.and_then(|mut ui_debug_config| {
				f(&mut ui_debug_config);
				Ok(())
			})
			.unwrap();
	}
	pub fn read_then(f: &mut dyn for<'a> FnMut(&'a UiDebugConfig)) {
		UIDEBUGCONFIG
			.read()
			.and_then(|ui_debug_config| {
				f(&ui_debug_config);
				Ok(())
			})
			.unwrap();
	}
}

static UIDEBUGCONFIG: Lazy<Arc<RwLock<UiDebugConfig>>> =
	Lazy::new(|| Arc::new(RwLock::new(UiDebugConfig::default())));
