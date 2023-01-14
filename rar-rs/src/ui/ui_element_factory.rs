use std::collections::HashMap;

use tracing::*;

use crate::ui::UiElement;
use crate::ui::UiElementInfo;

#[derive(Default)]
pub struct UiElementFactory {
	producer_fns: HashMap<&'static str, &'static dyn Fn() -> Box<dyn UiElement>>,
}

impl UiElementFactory {
	pub fn with_standard_ui_elements(mut self) -> Self {
		self.register_producer_via_info(&crate::ui::UiButton::info());
		self.register_producer_via_info(&crate::ui::UiToggleButton::info());
		self.register_producer_via_info(&crate::ui::UiSpacer::info());
		self.register_producer_via_info(&crate::ui::UiGridBox::info());
		self.register_producer_via_info(&crate::ui::UiLabel::info());
		self.register_producer_via_info(&crate::ui::UiImage::info());
		self.register_producer_via_info(&crate::ui::Ui3x3Image::info());
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
		warn!("No producer for {}", &element_type);
		None
	}
}

impl core::fmt::Debug for UiElementFactory {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
		write!(f, "UiElementFactory")
	}
}
