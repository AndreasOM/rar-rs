use oml_game::system::System;

use crate::omscript::Literal;
use crate::omscript::OpCode;
use crate::omscript::Script;

pub trait ScriptFunction {
	fn call(&mut self, params: Vec<&Literal>) -> bool;
	fn tick(&mut self) -> bool;
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
		write!(f, "[ScriptFunction]")
	}
}

impl core::fmt::Debug for dyn ScriptFunction {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
		self.fmt(f)
	}
}

#[derive(Debug, Default)]
struct ScriptFunctionWaitFrames {
	frames:        i128,
	target_frames: i128,
}

impl ScriptFunctionWaitFrames {}

impl ScriptFunction for ScriptFunctionWaitFrames {
	fn call(&mut self, params: Vec<&Literal>) -> bool {
		if params.len() != 1 {
			false
		} else {
			if let Literal::I128(n) = params[0] {
				self.frames = 0;
				self.target_frames = *n;
				true
			} else {
				false
			}
		}
	}
	fn tick(&mut self) -> bool {
		self.frames += 1;
		self.frames >= self.target_frames
	}
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
		write!(
			f,
			"ScriptFunctionWaitFrames {} >= {}",
			self.frames, self.target_frames
		)
	}
}

#[derive(Debug, Default)]
struct ScriptState {
	pub pc:               usize,
	pub running_function: Option<Box<dyn ScriptFunction>>,
}

#[derive(Debug, Default)]
pub struct ScriptVm {
	script:       Option<Script>,
	script_state: Option<ScriptState>,
}

impl ScriptVm {
	pub fn load(&mut self, system: &mut System, name: &str) -> anyhow::Result<()> {
		let s = Script::from_asset(system, name)?;
		tracing::debug!("Loaded script {:#?}", &s);
		self.script = Some(s);
		Ok(())
	}

	pub fn run(&mut self) -> anyhow::Result<()> {
		if let Some(script) = &self.script {
			if let Some(pc) = script.find_label("run") {
				tracing::debug!("Starting script at {}", pc);
				let mut ss = ScriptState::default();
				ss.pc = pc;
				self.script_state = Some(ss);
			} else {
			}
		} else {
		}
		Ok(())
	}

	fn call(&self, name: &str, params: Vec<&Literal>) -> Option<Box<dyn ScriptFunction>> {
		match name {
			"wait_frames" => {
				let mut f = ScriptFunctionWaitFrames::default();
				if !f.call(params) {
				} else {
					return Some(Box::new(f));
				}
			},
			n => {
				tracing::warn!("function not found {}", n);
				todo!();
			},
		}
		None
	}
	pub fn tick(&mut self) -> anyhow::Result<()> {
		tracing::debug!("Script::tick {:?}", &self);
		let mut next_function = None;
		let mut done = false;

		{
			if let Some(script_state) = &mut self.script_state {
				if let Some(running_function) = &mut script_state.running_function {
					if running_function.tick() {
						script_state.running_function = None;
					}
				} else {
					if let Some(script) = &self.script {
						if let Some(op_code) = script.get_op_code(script_state.pc) {
							match op_code {
								OpCode::Fn(_) => {},      // skip
								OpCode::BlockStart => {}, // :TODO: push scope
								OpCode::BlockEnd => {},   // :TODO: push scope
								OpCode::Call(ident, n) => {
									let n = *n;
									if let Some(l) = script.get_literal(*ident as usize) {
										if let Literal::STRING(name) = l {
											let mut params = Vec::new();
											for _ in 0..n {
												script_state.pc += 1;
												if let Some(sub_op_code) =
													script.get_op_code(script_state.pc)
												{
													match sub_op_code {
														OpCode::Literal(l) => {
															if let Some(l) =
																script.get_literal(*l as usize)
															{
																params.push(l);
															}
														},
														_ => unreachable!(),
													}
												} else {
													unreachable!();
												}
											}

											tracing::debug!("Calling {} with {:#?}", name, params);
											next_function = Some((name, params));
										} else {
											unreachable!();
										}
									} else {
										unreachable!();
									}
								},
								_ => unreachable!(),
							};
							script_state.pc += 1;
						} else {
							// done
							self.script_state = None;
							done = true;
						}
					} else {
					}
				}
			} else {
			}
		}
		if let Some((name, params)) = next_function {
			if let Some(f) = self.call(name, params) {
				if let Some(script_state) = &mut self.script_state {
					script_state.running_function = Some(f);
				}
			}
		}

		if done {
			self.script = None;
		}
		Ok(())
	}

	pub fn is_script_running(&self) -> bool {
		self.script.is_some()
	}
}
