use std::any::Any;

use oml_game::math::Matrix32;
use oml_game::math::Rectangle;
use oml_game::renderer::Renderer;

use tracing::*;

use crate::rar::effect_ids::EffectId;
use crate::rar::game_state::GameStateResponse;
use crate::rar::layer_ids::LayerId;
use crate::rar::AppUpdateContext;
use crate::rar::GameState;
use crate::rar::GameStateResponseDataSelectWorld;

#[derive(Debug, Default)]
pub struct GameStateMenu {
	buttons: Vec<(&'static str, Rectangle)>,
}

impl GameStateMenu {
	pub fn new() -> Self {
		Self {
			buttons: [
				("dev", (-128.0,-64.0,256.0, 64.0).into() ),
				("debug", (-128.0,64.0,256.0, 64.0).into() ),
			].to_vec(),
			..Default::default()
		}
	}
}


impl GameState for GameStateMenu {
	fn update(&mut self, auc: &mut AppUpdateContext) -> Vec<GameStateResponse> {
		let mut responses = Vec::new();

		let wuc = match auc.wuc() {
			Some(wuc) => wuc,
			None => return Vec::new(),
		};

		if wuc.was_mouse_button_pressed(0) {
			debug!("{}", auc.cursor_pos().y);
			for b in self.buttons.iter() {
				let r = &b.1;
				if r.contains( auc.cursor_pos() ) {
					debug!("Clicked {}", &b.0);
					let world = b.0;
					let sw = GameStateResponseDataSelectWorld::new(world);
					let r = GameStateResponse::new("SelectWorld").with_data(Box::new(sw));
					responses.push(r);
					let r = GameStateResponse::new("StartGame");
					responses.push(r);
					continue;
				}
			}
/*
			let world = if auc.cursor_pos().y < 0.0 {
				"dev"
			} else {
				"debug"
			};

*/
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

		renderer.use_texture("ui-button");
		renderer.use_layer(LayerId::Ui as u8);
		renderer.use_effect(EffectId::Textured as u16);

		for b in self.buttons.iter() {
			let r = &b.1;

			renderer.render_textured_quad(&r.center(), &r.size());
		}
	}
	fn as_any(&self) -> &(dyn Any + 'static) {
		self
	}
	fn as_any_mut(&mut self) -> &mut (dyn Any + 'static) {
		self
	}
}
