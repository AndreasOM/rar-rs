use crate::omscript::Literal;
use crate::omscript::ScriptContext;
use crate::omscript::ScriptFunction;
use crate::rar::RarScriptContext;

#[derive(Debug, Default)]
pub struct RarScriptFunctionQueueScreenshot {
	name: String,
}

impl RarScriptFunctionQueueScreenshot {
	pub fn create() -> Box<dyn ScriptFunction> {
		Box::new(Self::default())
	}
}

impl ScriptFunction for RarScriptFunctionQueueScreenshot {
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
		script_context.script_context_as_mut_then::<RarScriptContext>(
			&|rsc: &mut RarScriptContext| {
				rsc.screenshots.push(format!("{}", self.name));
			},
		);
		true
	}
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
		write!(f, "RarScriptFunctionQueueScreenshot {}", self.name)
	}
}
