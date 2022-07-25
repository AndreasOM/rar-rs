use oml_game::renderer::debug_renderer::DebugRenderer;
use oml_game::renderer::Renderer;
use oml_game::system::System;
use oml_game::window::WindowUpdateContext;

pub trait GameState {
	fn setup(&mut self, _system: &mut System) -> anyhow::Result<()> {
		Ok(())
	}
	fn teardown(&mut self) {}
	fn update(&mut self, _wuc: &mut WindowUpdateContext) {}
	fn render(&mut self, renderer: &mut Renderer) {}
	fn render_debug(&mut self, debug_renderer: &mut DebugRenderer) {}
	fn name(&self) -> &str {
		"[trait] GameState"
	}
}
