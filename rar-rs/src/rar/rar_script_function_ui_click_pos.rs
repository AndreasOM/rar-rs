use oml_game::math::Vector2;

use crate::omscript::Literal;
use crate::omscript::ScriptContext;
use crate::omscript::ScriptFunction;
use crate::omscript::ScriptFunctionCreator;
use crate::rar::RarScriptContext;

#[derive(Debug, Default)]
pub struct RarScriptFunctionUiClickPos {
	x: i128,
	y: i128,
}

impl RarScriptFunctionUiClickPos {
	pub fn create() -> Box<dyn ScriptFunction<RarScriptContext<'static>>> {
		Box::new(Self::default())
	}
}

impl ScriptFunction<RarScriptContext<'_>> for RarScriptFunctionUiClickPos {
	fn call(&mut self, _script_context: &mut RarScriptContext, params: Vec<&Literal>) -> bool {
		if params.len() != 2 {
			false
		} else {
			match (params[0], params[1]) {
				(Literal::I128(x), Literal::I128(y)) => {
					self.x = *x;
					self.y = *y;
					true
				},
				_ => false,
			}
		}
	}
	fn tick(&mut self, script_context: &mut RarScriptContext) -> bool {
		//script_context.ui_click_names.push(self.name.clone());
		script_context
			.ui_click_positions
			.push(Vector2::new(self.x as f32, self.y as f32));
		true
	}
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
		write!(f, "RarScriptFunctionUiClickPos {}, {}", self.x, self.y)
	}
}

#[derive(Debug, Default)]
pub struct RarScriptFunctionUiClickPosCreator {}

impl<C: ScriptContext> ScriptFunctionCreator<C> for RarScriptFunctionUiClickPosCreator
where
	RarScriptFunctionUiClickPos: ScriptFunction<C>,
{
	fn create(&self) -> Box<(dyn ScriptFunction<C> + 'static)> {
		Box::new(RarScriptFunctionUiClickPos::default())
	}
}
