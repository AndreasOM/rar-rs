use crate::ui::UiUpdateContext;

#[derive(Debug, Default)]
pub struct RarUiUpdateContext {}

impl RarUiUpdateContext {}

impl UiUpdateContext for RarUiUpdateContext {
	fn as_any(&self) -> &dyn std::any::Any {
		self
	}
	fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
		self
	}
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
		writeln!(f, "RarUiUpdateContext")
	}
}
