use crate::omscript::Literal;
use crate::omscript::ScriptContext;
use crate::omscript::ScriptFunction;
use crate::omscript::ScriptFunctionCreator;
use crate::rar::RarScriptContext;

#[derive(Debug, Default)]
pub struct RarScriptFunctionAppQuit {}

impl RarScriptFunctionAppQuit {
	pub fn create() -> Box<dyn ScriptFunction<RarScriptContext<'static>>> {
		Box::new(Self::default())
	}
}

impl ScriptFunction<RarScriptContext<'_>> for RarScriptFunctionAppQuit {
	fn call(&mut self, _script_context: &mut RarScriptContext, _params: Vec<&Literal>) -> bool {
		true
	}
	fn tick(&mut self, script_context: &mut RarScriptContext) -> bool {
		script_context.quit = true;
		true
	}
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
		write!(f, "RarScriptFunctionAppQuit")
	}
}

#[derive(Debug, Default)]
pub struct RarScriptFunctionAppQuitCreator {}

impl<C: ScriptContext> ScriptFunctionCreator<C> for RarScriptFunctionAppQuitCreator
where
	RarScriptFunctionAppQuit: ScriptFunction<C>,
{
	fn create(&self) -> Box<(dyn ScriptFunction<C> + 'static)> {
		Box::new(RarScriptFunctionAppQuit::default())
	}
}
