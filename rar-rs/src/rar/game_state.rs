use oml_game::renderer::debug_renderer::DebugRenderer;
use oml_game::renderer::Renderer;
use oml_game::system::System;
use oml_game::window::WindowUpdateContext;

use tracing::*;

pub trait GameState {
	fn setup(&mut self, _system: &mut System) -> anyhow::Result<()> {
		Ok(())
	}
	fn teardown(&mut self) {}
	fn update(&mut self, _wuc: &mut WindowUpdateContext) -> Vec< GameStateResponse > { Vec::new() }
	fn render(&mut self, _renderer: &mut Renderer) {}
	fn render_debug(&mut self, _debug_renderer: &mut DebugRenderer) {}
	fn name(&self) -> &str {
		"[trait] GameState"
	}
}

impl std::fmt::Debug for dyn GameState {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
		writeln!(
			f,
			"[Trait] GameState: {}",
			self.name(),
		)
	}
}

pub trait GameStateResponseData {
	fn name(&self) -> &str;
	fn as_any(&self) -> &dyn std::any::Any;
}

pub fn get_game_state_response_data_as_specific<'a, T: 'a + 'static>( gsrd: &'a dyn GameStateResponseData ) -> Option<&'a T> {
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
		writeln!(
			f,
			"[Trait] GameStateResponseData: {}",
			self.name(),
		)
	}
}

#[derive(Debug,Default)]
pub struct GameStateResponse {
	name: String,
	data: Option< Box< dyn GameStateResponseData > >,
}

impl GameStateResponse {
	pub fn new( name: &str ) -> Self {
		Self {
			name: name.to_string(),
			..Default::default()
		}
	}

	pub fn name( &self ) -> &str {
		&self.name
	}
}
