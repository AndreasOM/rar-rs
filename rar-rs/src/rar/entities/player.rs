use oml_game::math::Matrix22;
use oml_game::math::Vector2;
use oml_game::renderer::{AnimatedTexture, Color, Renderer};

use rand::prelude::*;

use crate::rar::effect_ids::EffectId;
use crate::rar::entities::Entity;
use crate::rar::entities::EntityConfiguration;
use crate::rar::entities::EntityData;
use crate::rar::entities::EntityType;
use crate::rar::layer_ids::LayerId;
use crate::rar::EntityUpdateContext;


#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum PlayerState {
	WaitForStart,
	Idle,
	Dying,
	Dead,
}

#[derive(Debug, PartialEq, Eq)]
enum PlayerDirection {
	Left,
	Right,
}

#[derive(Debug)]
pub struct Player {
	name: String,
	spawn_pos: Vector2,
	pos: Vector2,
	size: Vector2,
	state: PlayerState,
	direction: PlayerDirection,
	speed: f32,
	movement: Vector2,
	time_since_dying: f32,
	input_context_index: u8,
	animated_texture_idle_right: AnimatedTexture,
	animated_texture_idle_left: AnimatedTexture,
	animated_texture_dying: AnimatedTexture,
	entity_data: EntityData,
}

impl Player {
	pub fn new() -> Self {
		Self {
			name: "player".to_string(),
			spawn_pos: Vector2::new(0.0, 0.0),
			pos: Vector2::zero(),
			size: Vector2::new(128.0, 128.0),
			state: PlayerState::Dead,
			direction: PlayerDirection::Right,
			speed: 0.0,
			movement: Vector2::zero(),
			time_since_dying: f32::MAX,
			input_context_index: 0xff,
			animated_texture_idle_right: AnimatedTexture::new(),
			animated_texture_idle_left: AnimatedTexture::new(),
			animated_texture_dying: AnimatedTexture::new(),
			entity_data: EntityData::default(),
		}
	}

	pub fn name(&self) -> &str {
		&self.name
	}

	pub fn is_alive(&self) -> bool {
		match self.state {
			PlayerState::Dead | PlayerState::Dying => false,
			PlayerState::WaitForStart | PlayerState::Idle => true,
		}
	}

	pub fn can_respawn(&self) -> bool {
		self.state == PlayerState::Dead
	}

	pub fn movement(&self) -> &Vector2 {
		&self.movement
	}

	pub fn state(&self) -> PlayerState {
		self.state
	}

	fn goto_state(&mut self, state: PlayerState) {
		match state {
			PlayerState::WaitForStart => {
				self.pos = self.spawn_pos;
//				let o: f32 = random();
				let o = self.input_context_index as f32;
				println!("{}", o);
				self.pos.x += o*200.0 - 100.0;
				self.direction = PlayerDirection::Right;
			},
			PlayerState::Idle => {},
			PlayerState::Dying => {
				self.time_since_dying = 0.0;
			},
			_ => {},
		}
		self.state = state; // :TODO: handle transitions if needed
	}

	pub fn respawn(&mut self) {
		match self.state {
			PlayerState::Dead => {
				self.goto_state(PlayerState::WaitForStart);
			},
			_ => {},
		}
	}

	pub fn kill(&mut self) {
		if self.is_alive() {
			self.goto_state(PlayerState::Dying);
		}
	}

	fn update_waiting_for_start(&mut self, euc: &mut EntityUpdateContext) {
		// :TODO: move to game state
		//		self.animated_texture.update( euc.time_step() );
		//		self.movement.x = 0.0;
		if let Some( mut pic ) = euc.player_input_context( self.input_context_index ) {
			if pic.is_left_pressed || pic.is_right_pressed {
				self.goto_state(PlayerState::Idle); // :TODO: start logic
			}
		}
	}

	fn update_idle(&mut self, euc: &mut EntityUpdateContext) {
		if let Some( mut pic ) = euc.player_input_context( self.input_context_index ) {
			if pic.is_left_pressed {
				self.speed = -100.0;
				self.direction = PlayerDirection::Left;
			} else if pic.is_right_pressed {
				self.speed = 100.0;
				self.direction = PlayerDirection::Right;
			} else {
				self.speed = 0.0;
			}
		}
		self.animated_texture_idle_right.update(euc.time_step());
		self.animated_texture_idle_left.update(euc.time_step());
		self.movement.x = self.speed * euc.time_step() as f32;

		self.pos = self.pos.add(&self.movement);
	}

	pub fn set_pos(&mut self, pos: &Vector2) {
		self.pos = *pos;
	}

	pub fn pos(&self) -> &Vector2 {
		&self.pos
	}

	pub fn radius(&self) -> f32 {
		self.size.length() * 0.5
	}

	pub fn set_input_context_index( &mut self, index: u8 ) {
		self.input_context_index = index;
	}
}

impl Entity for Player {
	fn data(&self) -> &EntityData {
		&self.entity_data
	}
	fn as_any(&self) -> &dyn std::any::Any {
		self
	}
	fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
		self
	}

	fn setup(&mut self, _ec: &EntityConfiguration) {
		// self.name = name.to_owned();
		self.animated_texture_idle_right
			.setup("player-idle-right-", 4, 0, 8, 25.0);
		self.animated_texture_idle_left
			.setup("player-idle-left-", 4, 0, 8, 25.0);
		//		self.animated_texture_dying.setup( "fish_die", 2, 0, 2, 25.0 );
		//		self.animated_texture.setup_from_config( &ec.animated_texture_configuration );
	}

	fn teardown(&mut self) {}

	fn update(&mut self, euc: &mut EntityUpdateContext) {
		//		println!("Player: {:?}", &self);
		// :TODO: time step
		match self.state {
			PlayerState::WaitForStart => self.update_waiting_for_start(euc),
			PlayerState::Idle => self.update_idle(euc),
			_ => {},
		}

		if let Some(debug_renderer) = &*euc.debug_renderer {
			let mut debug_renderer = debug_renderer.borrow_mut();
			let color = Color::from_rgba(0.8, 0.6, 0.3, 0.8);
			debug_renderer.add_line(&self.pos, &Vector2::zero(), 1.0, &color);
			debug_renderer.add_frame(&self.pos, &self.size, 5.0, &color);
			let target = &Vector2::new(250.0, 0.0);

			let target = self.pos.add(&target);
			debug_renderer.add_line(&self.pos, &target, 3.0, &color);

			//			let radius = self.size.length() * 0.5;
			//			debug_renderer.add_circle( &self.pos, radius, 5.0, &color );
		}
	}

	fn render(&mut self, renderer: &mut Renderer) {
		if self.state == PlayerState::Dead {
			// dead means offscreen, nothing to be rendered
			return;
		}

		renderer.use_layer(LayerId::Player as u8);
		renderer.use_effect(EffectId::Textured as u16);
		match self.state {
			PlayerState::Dying | PlayerState::Dead => self.animated_texture_dying.r#use(renderer),
			_ => match self.direction {
				PlayerDirection::Right => self.animated_texture_idle_right.r#use(renderer),
				PlayerDirection::Left => self.animated_texture_idle_left.r#use(renderer),
			},
		}
		renderer.render_textured_quad(&self.pos, &self.size);
	}

	fn name(&self) -> &str {
		&self.name
	}

	fn entity_type(&self) -> EntityType {
		EntityType::Player
	}
}
