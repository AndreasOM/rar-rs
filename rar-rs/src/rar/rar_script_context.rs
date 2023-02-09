use crate::omscript::ScriptContext;
use crate::ui::UiSystem;
//use crate::rar::GameState;

#[derive(Default, Debug)]
pub struct RarScriptContext<'a> {
	pub quit:        bool,
	pub screenshots: Vec<String>,
	pub ui_system:   Option<&'a UiSystem>, // :TODO:
	//pub game_state: Option<&'a  Box< dyn GameState >>,
	// :HACK:
	pub ui_click_names: Vec<String>,
}
impl ScriptContext for RarScriptContext<'_> {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
		write!(f, "RarScriptContext") // :TODO: add fields
	}
}
