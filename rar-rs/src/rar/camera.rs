use derive_getters::Getters;

use oml_game::math::Vector2;

#[derive(Debug,Default,Getters)]
pub struct Camera {
	pos: Vector2,
	target_pos: Vector2,
}

impl Camera {

	pub fn set_target_pos( &mut self, target_pos: &Vector2 ) {
		self.target_pos = *target_pos;
	}

	pub fn offset( &self ) -> Vector2 {
		self.pos.scaled_vector2( &Vector2::new( -1.0, -1.0 ) )
	}

	pub fn update(&mut self, time_step: f64) {
		let ls = 1.1 * time_step as f32;
		self.pos.x = lerp( self.pos.x, self.target_pos.x, ls);
		self.pos.y = lerp( self.pos.y, self.target_pos.y, ls);

	}	
}

fn lerp(v0: f32, v1: f32, t: f32) -> f32 {
  (1.0 - t) * v0 + t * v1
}


