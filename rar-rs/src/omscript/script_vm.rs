use core::marker::PhantomData;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::RwLock;

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

/*
#[derive(Debug, Default)]
struct ScriptState<C: ScriptContext> {
	pub pc:               usize,
	pub running_function: Option<Box<dyn ScriptFunction<C>>>,
	pub running:          bool,
	pub pc_stack:         Vec<usize>,
}
*/

//type ScriptFunctionCreator<C> = fn() -> Box<dyn ScriptFunction<C>>;

#[derive(Debug, Default)]
struct ScriptRunner<C: ScriptContext> {
	pub pc:                   usize,
	pub running_function:     Option<Box<dyn ScriptFunction<C>>>,
	pub running:              bool,
	pub pc_stack:             Vec<usize>,
	script:                   Arc<Script>,
	script_function_creators: Arc<RwLock<HashMap<String, Box<dyn ScriptFunctionCreator<C>>>>>,
}

impl<C: ScriptContext + std::default::Default> ScriptRunner<C> {
	pub fn with_script(mut self, script: Arc<Script>) -> Self {
		self.script = script;
		self
	}

	pub fn with_script_function_creators(
		mut self,
		script_function_creators: &Arc<RwLock<HashMap<String, Box<dyn ScriptFunctionCreator<C>>>>>,
	) -> Self {
		self.script_function_creators = Arc::clone(script_function_creators);
		self
	}

	pub fn run(&mut self) {
		self.pc = 0;
		self.running = true;
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
			"debug" => {
				if params.len() == 1 {
					if let Literal::STRING(s) = &params[0] {
						tracing::debug!("Script Debug: >{}<", s);
					}
				}
			},
			n => {
				if let Some(creator) = self.script_function_creators.read().unwrap().get(n) {
					let mut f = creator.create();
					if !f.call(script_context, params) {
					} else {
						return Some(f);
					}
				}
			},
		}
		None
	}

	pub fn tick(&mut self, script_context: &mut C) -> anyhow::Result<()> {
		//tracing::debug!("ScriptRunner::tick {:?}", &self);
		let mut next_function = None;

		{
			if let Some(running_function) = &mut self.running_function {
				if running_function.tick(script_context) {
					self.running_function = None;
					self.pc_stack.pop();
				}
			} else {
				if let Some(op_code) = self.script.get_op_code(self.pc) {
					match op_code {
						OpCode::Fn(_) => {},      // skip
						OpCode::BlockStart => {}, // :TODO: push scope
						OpCode::BlockEnd => {
							tracing::debug!("BlockEnd @{} [{:?}]", self.pc, self.pc_stack);
							if let Some(pc) = self.pc_stack.pop() {
								self.pc = pc;
							} else {
								// we are done for good?
								todo!();
							}
						}, // :TODO: pop scope
						OpCode::Call(ident, n) => {
							let n = *n;
							if let Some(l) = self.script.get_literal(*ident as usize) {
								if let Literal::STRING(name) = l {
									let mut params = Vec::new();
									for _ in 0..n {
										self.pc += 1;
										if let Some(sub_op_code) = self.script.get_op_code(self.pc)
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
									self.pc_stack.push(self.pc);
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
					self.pc += 1;
				} else {
					self.running = false;
				}
			}
		}
		if let Some((name, params)) = next_function {
			if let Some(f) = self.call(script_context, name, params) {
				self.running_function = Some(f);
			} else {
				// find an fn
				if let Some(pc) = self.script.find_label(name) {
					tracing::debug!("Call target {} found at {}", name, pc);
					// :TODO: handle parameters
					self.pc = pc;
				} else {
					tracing::warn!("function not found {}", name);
					self.pc_stack.pop();
					//todo!();
				}
			}
		}

		Ok(())
	}
}

//#[derive(Debug, Default)]
pub struct ScriptRunnerWithContext<'a, C>
where
	C: ScriptContext + std::fmt::Debug,
{
	script_runner: &'a mut ScriptRunner<C>,
	script_context: &'a mut C,	
}

impl<C> ScriptRunnerWithContext<'_, C>
where
	C: ScriptContext + std::default::Default + std::fmt::Debug,
{
	pub fn tick(&mut self) -> anyhow::Result<()> {
		//tracing::debug!("ScriptRunnerWithContext::tick {:?}", &self);
		self.script_runner.tick(self.script_context)?;

		Ok(())
	}	
}

#[derive(Debug, Default)]
pub struct ScriptVm<C>
where
	C: ScriptContext + std::fmt::Debug,
{
	script:                   Arc<Script>,
	//script_state:             ScriptState<C>,
	script_function_creators: Arc<RwLock<HashMap<String, Box<dyn ScriptFunctionCreator<C>>>>>,
	phantom:                  PhantomData<C>,
	script_runner:            Option<Arc<RwLock<ScriptRunner<C>>>>,
	script_runner_2:			ScriptRunner<C>,
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
			.write()
			.unwrap()
			.insert(name.to_string(), creator);
	}
	pub fn load(&mut self, system: &mut System, name: &str) -> anyhow::Result<()> {
		let s = Script::from_asset(system, name)?;
		tracing::debug!("Loaded script {:#?}", &s);
		self.script = Arc::new(s);
		//self.script_state.running = false;
		Ok(())
	}

	pub fn run(&mut self) -> anyhow::Result<()> {
		if let Some(pc) = self.script.find_label("run") {
			tracing::debug!("Starting script at {}", pc);
			//let mut ss = ScriptState::default();
			//ss.pc = pc;
			//self.script_state = ss; // :TODO: we could just reset the existing one
			//self.script_state.running = true;
			let mut script_runner = ScriptRunner::default()
				.with_script(Arc::clone(&self.script))
				.with_script_function_creators(&self.script_function_creators);
			script_runner.run();

			self.script_runner = Some(Arc::new(RwLock::new(script_runner)));
		} else {
		}
		Ok(())
	}

	pub fn script_runner_with_context_mut<'a>(&'a mut self, script_context: &'a mut C) -> ScriptRunnerWithContext<'a,C>{
		ScriptRunnerWithContext {
			script_runner: &mut self.script_runner_2,
			script_context: script_context,
		}
	}

	pub fn tick(&self, script_context: &mut C) -> anyhow::Result<()> {
		//tracing::debug!("Script::tick {:?}", &self);
		if let Some(script_runner) = &self.script_runner {
			script_runner.write().unwrap().tick(script_context)?;
		}

		Ok(())
	}

	pub fn is_script_running(&self) -> bool {
		if let Some(script_runner) = &self.script_runner {
			return script_runner.read().unwrap().running;
		}
		false
	}
}
