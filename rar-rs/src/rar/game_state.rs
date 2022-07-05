use oml_game::renderer::Renderer;
use oml_game::window::WindowUpdateContext;
use oml_game::system::System;

pub trait GameState {
	fn setup(&mut self, _system: &mut System) {}
	fn teardown(&mut self) {}
	fn update(&mut self, _wuc: &mut WindowUpdateContext) {}
	fn render(&mut self, renderer: &mut Renderer) {}
	fn name(&self) -> &str {
		"[trait] GameState"
	}
}
