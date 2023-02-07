use oml_game::system::System;

use crate::omscript::Script;

#[derive(Debug, Default)]
pub struct ScriptVm {
	script: Option<Script>,
}

impl ScriptVm {
	pub fn load(&mut self, system: &mut System, name: &str) -> anyhow::Result<()> {
		let s = Script::from_asset(system, name)?;
		tracing::debug!("Loaded script {:#?}", &s);
		Ok(())
	}

	pub fn run(&mut self) {}

	pub fn tick(&mut self) {}

	pub fn is_script_running(&self) -> bool {
		self.script.is_some()
	}
}
