use oml_game::math::Matrix32;
use oml_game::renderer::Renderer;

use crate::rar::camera::Camera;
use crate::rar::effect_ids::EffectId;
use crate::rar::entities::{Entity, EntityConfiguration, EntityData, EntityType};
use crate::rar::layer_ids::LayerId;
use crate::rar::EntityUpdateContext;

#[derive(Debug)]
pub struct Background {
	name: String,

	entity_data: EntityData,
}

impl Background {
	pub fn new() -> Self {
		Self {
			name:        "background".to_string(),
			entity_data: EntityData::default(),
		}
	}

	pub fn name(&self) -> &str {
		&self.name
	}
}

impl Entity for Background {
	fn data(&self) -> &EntityData {
		&self.entity_data
	}
	fn as_any(&self) -> &dyn std::any::Any {
		self
	}
	fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
		self
	}

	fn setup(&mut self, _ec: &EntityConfiguration) {}

	fn teardown(&mut self) {}

	fn update(&mut self, _euc: &mut EntityUpdateContext) {}

	fn render(&mut self, renderer: &mut Renderer, _camera: &Camera) {
		renderer.use_texture("bg-title");
		renderer.use_layer(LayerId::Background as u8);
		renderer.use_effect(EffectId::Background as u16);

		let a = renderer.aspect_ratio();
		let mtx = Matrix32::scaling_xy(1.0 * a, 1.0);
		//mtx.pos.x = - self.pos.x / 1024.0;
		renderer.set_tex_matrix(&mtx);

		renderer.render_textured_fullscreen_quad();

		renderer.set_tex_matrix(&Matrix32::identity());
	}
	fn name(&self) -> &str {
		&self.name
	}

	fn entity_type(&self) -> EntityType {
		EntityType::Decoration
	}
}
