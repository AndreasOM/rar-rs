use crate::omscript::Literal;
use crate::omscript::ScriptContext;
use crate::omscript::ScriptFunction;
use crate::rar::RarScriptContext;

#[derive(Debug, Default)]
pub struct RarScriptFunctionAppQuit {}

impl RarScriptFunctionAppQuit {
	pub fn create() -> Box<dyn ScriptFunction> {
		Box::new(Self::default())
	}
}

impl ScriptFunction for RarScriptFunctionAppQuit {
	fn call(&mut self, _script_context: &mut dyn ScriptContext, _params: Vec<&Literal>) -> bool {
		true
	}
	fn tick(&mut self, script_context: &mut dyn ScriptContext) -> bool {
		script_context.script_context_as_mut_then::<RarScriptContext>(
			&|rsc: &mut RarScriptContext| {
				rsc.quit = true;
			},
		);
		true
	}
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
		write!(f, "RarScriptFunctionAppQuit")
	}
}
