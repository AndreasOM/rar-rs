use std::collections::HashMap;

use tracing::*;

use crate::ui::UiElement;
use crate::ui::UiElementInfo;

#[derive(Default)]
pub struct UiElementFactory {
	producer_fns: HashMap<&'static str, &'static dyn Fn() -> Box<dyn UiElement>>,
}

impl UiElementFactory {
	pub fn with_standard_ui_elements( mut self ) -> Self {
		self
	}
	
	pub fn register_producer_via_info(&mut self, info: &UiElementInfo) {
		let t = info.type_name;
		let f = info.producer_fn;
		self.producer_fns.insert(t, f);
	}

	pub fn produce_element(&self, element_type: &str) -> Option<Box<dyn UiElement>> {
		if let Some(f) = self.producer_fns.get(element_type) {
			return Some(f());
		};
		debug!("{:?}", &self);
		None
	}
}

impl core::fmt::Debug for UiElementFactory {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
		write!(f, "UiElementFactory")
	}
}
