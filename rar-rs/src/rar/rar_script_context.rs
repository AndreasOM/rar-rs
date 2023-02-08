//use crate::ui::UiSystem;
use crate::omscript::ScriptContext;

#[derive(Default)]
pub struct RarScriptContext {
	pub quit:        bool,
	pub screenshots: Vec<String>,
	//	pub ui_system: Option< &UiSystem >,
}
impl ScriptContext for RarScriptContext {
	fn as_any(&self) -> &dyn std::any::Any {
		self
	}
	fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
		self
	}
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
		write!(f, "RarScriptContext") // :TODO: add fields
	}
}
