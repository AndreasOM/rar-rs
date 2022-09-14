use oml_game::renderer::debug_renderer::DebugRenderer;
use oml_game::renderer::Renderer;
use oml_game::system::System;

//use oml_game::window::WindowUpdateContext;
//use tracing::*;
use crate::rar::AppUpdateContext;
/*
pub fn get_game_state_as_specific<'a, T: 'a + 'static>(
	gs: &'a Box<dyn GameState>,
) -> Option<&'a T> {
	match gs.as_any().downcast_ref::<T>() {
		Some(t) => Some(t),
		None => {
			//				warn!("{:?} isn't a {}!", &gs, std::any::type_name::<T>());
			None
		},
	}
}
*/

pub fn get_game_state_as_specific_mut<'a, T: 'a + 'static>(
	gs: &'a mut Box<dyn GameState>,
) -> Option<&'a mut T> {
	match gs.as_any_mut().downcast_mut::<T>() {
		Some(t) => Some(t),
		None => {
			//				warn!("{:?} isn't a {}!", &gs, std::any::type_name::<T>());
			None
		},
	}
}

pub trait GameState {
	fn as_any(&self) -> &dyn std::any::Any;
	fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
	fn setup(&mut self, _system: &mut System) -> anyhow::Result<()> {
		Ok(())
	}
	fn teardown(&mut self) {}
	fn update(&mut self, _wuc: &mut AppUpdateContext) -> Vec<GameStateResponse> {
		Vec::new()
	}
	fn render(&mut self, _renderer: &mut Renderer) {}
	fn render_debug(&mut self, _debug_renderer: &mut DebugRenderer) {}
	fn name(&self) -> &str {
		"[trait] GameState"
	}
}

impl std::fmt::Debug for dyn GameState {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
		writeln!(f, "[Trait] GameState: {}", self.name(),)
	}
}

pub trait GameStateResponseData {
	fn name(&self) -> &str;
	fn as_any(&self) -> &dyn std::any::Any;
}

pub fn get_game_state_response_data_as_specific<'a, T: 'a + 'static>(
	gsrd: &'a Box<dyn GameStateResponseData>,
) -> Option<&'a T> {
	match gsrd.as_any().downcast_ref::<T>() {
		Some(t) => Some(t),
		None => {
			//				warn!("{:?} isn't a {}!", &gsrd, std::any::type_name::<T>());
			None
		},
	}
}

impl std::fmt::Debug for dyn GameStateResponseData {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
		writeln!(f, "[Trait] GameStateResponseData: {}", self.name(),)
	}
}

#[derive(Debug, Default)]
pub struct GameStateResponse {
	name: String,
	data: Option<Box<dyn GameStateResponseData>>,
}

impl GameStateResponse {
	pub fn new(name: &str) -> Self {
		Self {
			name: name.to_string(),
			..Default::default()
		}
	}

	pub fn with_data(mut self, data: Box<dyn GameStateResponseData>) -> Self {
		self.data = Some(data);
		self
	}

	pub fn name(&self) -> &str {
		&self.name
	}

	pub fn data(&self) -> &Option<Box<dyn GameStateResponseData>> {
		&self.data
	}
}
