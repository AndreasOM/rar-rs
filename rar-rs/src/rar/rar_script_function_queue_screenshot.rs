use crate::omscript::Literal;
use crate::omscript::ScriptContext;
use crate::omscript::ScriptFunction;
use crate::omscript::ScriptFunctionCreator;
use crate::rar::RarScriptContext;

#[derive(Debug, Default)]
pub struct RarScriptFunctionQueueScreenshot {
	name: String,
}

impl ScriptFunction<RarScriptContext<'_>> for RarScriptFunctionQueueScreenshot {
	fn call(&mut self, _script_context: &mut RarScriptContext, params: Vec<&Literal>) -> bool {
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
	fn tick(&mut self, script_context: &mut RarScriptContext) -> bool {
		script_context.screenshots.push(format!("{}", self.name));
		true
	}
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
		write!(f, "RarScriptFunctionQueueScreenshot {}", self.name)
	}
}

#[derive(Debug, Default)]
pub struct RarScriptFunctionQueueScreenshotCreator {}

impl<C: ScriptContext> ScriptFunctionCreator<C> for RarScriptFunctionQueueScreenshotCreator
where
	RarScriptFunctionQueueScreenshot: ScriptFunction<C>,
{
	fn create(&self) -> Box<(dyn ScriptFunction<C> + 'static)> {
		Box::new(RarScriptFunctionQueueScreenshot::default())
	}
}
