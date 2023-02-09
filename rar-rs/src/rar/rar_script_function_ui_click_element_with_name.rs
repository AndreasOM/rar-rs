use crate::omscript::Literal;
use crate::omscript::ScriptContext;
use crate::omscript::ScriptFunction;
use crate::omscript::ScriptFunctionCreator;
use crate::rar::RarScriptContext;

#[derive(Debug, Default)]
pub struct RarScriptFunctionUiClickElementWithName {
	name: String,
}

impl RarScriptFunctionUiClickElementWithName {
	pub fn create() -> Box<dyn ScriptFunction<RarScriptContext<'static>>> {
		Box::new(Self::default())
	}
}

impl ScriptFunction<RarScriptContext<'_>> for RarScriptFunctionUiClickElementWithName {
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
		script_context.ui_click_names.push(self.name.clone());
		/*
		if let Some( ui_system ) = script_context.ui_system {
			if let Some( root ) = ui_system.root() {
				if !root.find_child_container_by_name_then(
					&self.name,
					&|c| {
						tracing::debug!( "Found {} -> {:?}", &self.name, &c );
					}
				) {
					tracing::warn!("Didn't find {}", &self.name );
				}
			} else {
				tracing::warn!("No Root to click -> {}", &self.name );
			}

		} else {
			tracing::warn!("No UiSystem to click -> {}", &self.name );
		}
		*/
		true
	}
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
		write!(f, "RarScriptFunctionUiClickElementWithName {}", self.name)
	}
}

#[derive(Debug, Default)]
pub struct RarScriptFunctionUiClickElementWithNameCreator {}

impl<C: ScriptContext> ScriptFunctionCreator<C> for RarScriptFunctionUiClickElementWithNameCreator
where
	RarScriptFunctionUiClickElementWithName: ScriptFunction<C>,
{
	fn create(&self) -> Box<(dyn ScriptFunction<C> + 'static)> {
		Box::new(RarScriptFunctionUiClickElementWithName::default())
	}
}
