use std::collections::HashMap;

use tracing::*;

use crate::ui::UiElement;
use crate::ui::UiElementInfo;

#[derive(Default)]
pub struct UiElementFactory {
	producers:    HashMap<&'static str, Box<dyn UiElementProducer>>,
	producer_fns: HashMap<&'static str, &'static dyn Fn() -> Box<dyn UiElement>>,
}

impl UiElementFactory {
	pub fn register_producer(&mut self, producer: Box<dyn UiElementProducer>) {
		let t = producer.produces_type();
		self.producers.insert(t, producer);
	}

	pub fn register_producer_via_info(&mut self, info: &UiElementInfo) {
		let t = info.type_name;
		let f = info.producer_fn;
		self.producer_fns.insert(t, f);
	}

	pub fn produce_element(&self, element_type: &str) -> Option<Box<dyn UiElement>> {
		if let Some(p) = self.producers.get(element_type) {
			return Some(p.produce());
		};
		if let Some(f) = self.producer_fns.get(element_type) {
			return Some(f());
		};
		debug!("{:?}", &self);
		None

		/*

		let element: Box<dyn UiElement> = match element_type {
			//"UiButton" => Box::new(crate::ui::UiButton::default()),
			//"UiToggleButton" => Box::new(crate::ui::UiToggleButton::default()),
			//"UiSpacer" => Box::new(crate::ui::UiSpacer::default()),
			//"UiGridBox" => Box::new(crate::ui::UiGridBox::default()),
			//"UiLabel" => Box::new(crate::ui::UiLabel::default()),
			//"UiImage" => Box::new(crate::ui::UiImage::default()),
			t => {
				error!("Producing from not supported for {}", &t);
				return None;
			},
		};
		debug!("{:?}", &element);
		Some(element)
		*/
	}
}

impl core::fmt::Debug for UiElementFactory {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
		write!(f, "UiElementFactory")
	}
}
pub trait UiElementProducer {
	fn produces_type(&self) -> &'static str;
	fn produce(&self) -> Box<dyn UiElement>;
}
impl std::fmt::Debug for dyn UiElementProducer {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
		writeln!(f, "[Trait] UiElementProducer")
	}
}
