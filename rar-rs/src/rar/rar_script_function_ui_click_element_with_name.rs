use crate::omscript::Literal;
use crate::omscript::ScriptContext;
use crate::omscript::ScriptFunction;

#[derive(Debug, Default)]
pub struct RarScriptFunctionUiClickElementWithName {
	name: String,
}

impl RarScriptFunctionUiClickElementWithName {
	pub fn create() -> Box<dyn ScriptFunction> {
		Box::new(Self::default())
	}
}

impl ScriptFunction for RarScriptFunctionUiClickElementWithName {
	fn call(&mut self, _script_context: &mut dyn ScriptContext, params: Vec<&Literal>) -> bool {
		if params.len() != 1 {
			false
		} else {
			if let Literal::STRING(s) = params[0] {
				self.name = s.clone();
				true
			} else {
				false
			}
		}
	}
	fn tick(&mut self, script_context: &mut dyn ScriptContext) -> bool {
		true
	}
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
		write!(f, "RarScriptFunctionUiClickElementWithName {}", self.name)
	}
}
