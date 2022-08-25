use derive_getters::Getters;
use oml_game::math::Rectangle;
use oml_game::math::Vector2;

use crate::rar::entities::{EntityId, EntityManager, Player};

#[derive(Debug, Default)]
pub enum CameraState {
	Frozen,
	#[default]
	Active,
}

#[derive(Debug, Default)]
pub enum CameraMode {
	FollowPlayerEntityId {
		id: EntityId,
	},
	#[default]
	None,
}

#[derive(Debug, Getters)]
pub struct Camera {
	pos:                 Vector2,
	target_pos:          Vector2,
	mode:                CameraMode,
	state:               CameraState,
	punch_factor:        f32,
	target_punch_factor: f32,
	frame_size:          Vector2,
}

impl Default for Camera {
	fn default() -> Self {
		Self {
			pos:                 Vector2::default(),
			target_pos:          Vector2::default(),
			mode:                CameraMode::default(),
			state:               CameraState::default(),
			punch_factor:        1.0,
			target_punch_factor: 1.0,
			frame_size:          Vector2::default(),
			//			..Default::default()
		}
	}
}
impl Camera {
	pub fn set_pos(&mut self, pos: &Vector2) {
		self.pos = *pos;
	}
	pub fn set_target_pos(&mut self, target_pos: &Vector2) {
		self.target_pos = *target_pos;
	}

	pub fn set_frame_size(&mut self, frame_size: &Vector2) {
		self.frame_size = *frame_size;
	}

	pub fn frame(&self) -> Rectangle {
		let f = Rectangle::default()
			.with_size(&self.frame_size)
			.with_center(&self.pos);

		f
	}
	pub fn offset(&self) -> Vector2 {
		self.pos.scaled_vector2(&Vector2::new(-1.0, -1.0))
	}

	pub fn scale(&self) -> f32 {
		self.punch_factor
	}
	pub fn follow_player_entity_id(&mut self, id: EntityId) {
		self.mode = CameraMode::FollowPlayerEntityId { id }
	}

	pub fn punch(&mut self, punch_factor: f32) {
		self.punch_factor = punch_factor;
	}

	pub fn thaw(&mut self) {
		self.state = CameraState::Active;
	}
	pub fn freeze(&mut self) {
		self.state = CameraState::Frozen;
	}

	pub fn update(&mut self, time_step: f64, entity_manager: &EntityManager) {
		match self.state {
			CameraState::Frozen => {
				return;
			},
			_ => {},
		};

		match self.mode {
			CameraMode::FollowPlayerEntityId { id } => {
				if let Some(p) = entity_manager.get_as::<Player>(id) {
					//println!("{:?}", p.pos());
					self.set_target_pos(p.pos());
				} else {
					panic!("Can not follow player {}", id);
				}
			},
			_ => {},
		}

		let ls = 1.1 * time_step as f32;

		self.punch_factor = lerp(self.punch_factor, self.target_punch_factor, ls);
		self.pos.x = lerp(self.pos.x, self.target_pos.x, ls);
		self.pos.y = lerp(self.pos.y, self.target_pos.y, ls);
	}
}

fn lerp(v0: f32, v1: f32, t: f32) -> f32 {
	(1.0 - t) * v0 + t * v1
}
