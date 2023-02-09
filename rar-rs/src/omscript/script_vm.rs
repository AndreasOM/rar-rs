use core::marker::PhantomData;
use std::collections::HashMap;

use oml_game::system::System;

use crate::omscript::Literal;
use crate::omscript::OpCode;
use crate::omscript::Script;
use crate::omscript::ScriptContext;

pub trait ScriptFunction<C>
where
	C: ScriptContext,
{
	fn call(&mut self, _script_context: &mut C, params: Vec<&Literal>) -> bool;
	fn tick(&mut self, _script_context: &mut C) -> bool;
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
		write!(f, "[ScriptFunction]")
	}
}

impl<C: ScriptContext> core::fmt::Debug for dyn ScriptFunction<C> {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
		self.fmt(f)
	}
}

pub trait ScriptFunctionCreator<C>: std::fmt::Debug
where
	C: ScriptContext,
{
	fn create(&self) -> Box<dyn ScriptFunction<C>>;
}

#[derive(Debug, Default)]
struct ScriptFunctionWaitFrames {
	frames:        i128,
	target_frames: i128,
}

impl ScriptFunctionWaitFrames {}

impl<C> ScriptFunction<C> for ScriptFunctionWaitFrames
where
	C: ScriptContext,
{
	fn call(&mut self, _script_context: &mut C, params: Vec<&Literal>) -> bool {
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
	fn tick(&mut self, _script_context: &mut C) -> bool {
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
struct ScriptState<C: ScriptContext> {
	pub pc:               usize,
	pub running_function: Option<Box<dyn ScriptFunction<C>>>,
	pub running:          bool,
}

//type ScriptFunctionCreator<C> = fn() -> Box<dyn ScriptFunction<C>>;

#[derive(Debug, Default)]
pub struct ScriptVm<C>
where
	C: ScriptContext + std::fmt::Debug,
{
	script:                   Script,
	script_state:             ScriptState<C>,
	script_function_creators: HashMap<String, Box<dyn ScriptFunctionCreator<C>>>,
	phantom:                  PhantomData<C>,
}

impl<C> ScriptVm<C>
where
	C: ScriptContext + std::default::Default + std::fmt::Debug,
{
	pub fn register_script_function(
		&mut self,
		name: &str,
		creator: Box<dyn ScriptFunctionCreator<C>>,
	) {
		self.script_function_creators
			.insert(name.to_string(), creator);
	}
	pub fn load(&mut self, system: &mut System, name: &str) -> anyhow::Result<()> {
		let s = Script::from_asset(system, name)?;
		tracing::debug!("Loaded script {:#?}", &s);
		self.script = s;
		self.script_state.running = false;
		Ok(())
	}

	pub fn run(&mut self) -> anyhow::Result<()> {
		if let Some(pc) = self.script.find_label("run") {
			tracing::debug!("Starting script at {}", pc);
			let mut ss = ScriptState::default();
			ss.pc = pc;
			self.script_state = ss; // :TODO: we could just reset the existing one
			self.script_state.running = true;
		} else {
		}
		Ok(())
	}

	fn call(
		&self,
		script_context: &mut C,
		name: &str,
		params: Vec<&Literal>,
	) -> Option<Box<dyn ScriptFunction<C>>> {
		match name {
			"wait_frames" => {
				let mut f = ScriptFunctionWaitFrames::default();
				if !f.call(script_context, params) {
				} else {
					return Some(Box::new(f));
				}
			},
			n => {
				if let Some(creator) = self.script_function_creators.get(n) {
					let mut f = creator.create();
					if !f.call(script_context, params) {
					} else {
						return Some(f);
					}
				} else {
					tracing::warn!("function not found {}", n);
					todo!();
				}
			},
		}
		None
	}
	pub fn tick(&mut self, script_context: &mut C) -> anyhow::Result<()> {
		// tracing::debug!("Script::tick {:?}", &self);
		let mut next_function = None;

		{
			if let Some(running_function) = &mut self.script_state.running_function {
				if running_function.tick(script_context) {
					self.script_state.running_function = None;
				}
			} else {
				if let Some(op_code) = self.script.get_op_code(self.script_state.pc) {
					match op_code {
						OpCode::Fn(_) => {},      // skip
						OpCode::BlockStart => {}, // :TODO: push scope
						OpCode::BlockEnd => {},   // :TODO: push scope
						OpCode::Call(ident, n) => {
							let n = *n;
							if let Some(l) = self.script.get_literal(*ident as usize) {
								if let Literal::STRING(name) = l {
									let mut params = Vec::new();
									for _ in 0..n {
										self.script_state.pc += 1;
										if let Some(sub_op_code) =
											self.script.get_op_code(self.script_state.pc)
										{
											match sub_op_code {
												OpCode::Literal(l) => {
													if let Some(l) =
														self.script.get_literal(*l as usize)
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
					self.script_state.pc += 1;
				} else {
					self.script_state.running = false;
				}
			}
		}
		if let Some((name, params)) = next_function {
			if let Some(f) = self.call(script_context, name, params) {
				self.script_state.running_function = Some(f);
			}
		}

		Ok(())
	}
	/*
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
	*/
	pub fn is_script_running(&self) -> bool {
		self.script_state.running
	}
}
