use oml_game::renderer::Renderer;
use oml_game::math::Matrix32;
use oml_game::window::WindowUpdateContext;

use crate::rar::game_state::GameStateResponse;
use crate::rar::GameState;

use crate::rar::effect_ids::EffectId;
use crate::rar::layer_ids::LayerId;

#[derive(Debug, Default)]
pub struct GameStateMenu {
}

impl GameStateMenu {
	pub fn new() -> Self {
		Self {
			..Default::default()
		}
	}	
}


impl GameState for GameStateMenu {
	fn update(&mut self, wuc: &mut WindowUpdateContext) -> Vec< GameStateResponse > { 

		let mut responses = Vec::new();

		if wuc.was_mouse_button_pressed( 0 ) {
			let r = GameStateResponse::new( "StartGame" );
			responses.push( r );
		}

		responses
	}
	fn render(&mut self, renderer: &mut Renderer) {
		renderer.use_texture("bg-menu");
		renderer.use_layer(LayerId::Background as u8);
		renderer.use_effect(EffectId::Background as u16);

		let a = renderer.aspect_ratio();
		let mtx = Matrix32::scaling_xy(1.0 * a, 1.0);
		//mtx.pos.x = - self.pos.x / 1024.0;
		renderer.set_tex_matrix(&mtx);

		renderer.render_textured_fullscreen_quad();

		renderer.set_tex_matrix(&Matrix32::identity());

	}
}

