use std::cell::Cell;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::convert::From;

use oml_game::math::{Cardinals, Rectangle, Vector2};
use oml_game::renderer::debug_renderer;
use oml_game::renderer::{AnimatedTexture, Color, Renderer};
use tracing::*;

use crate::rar::camera::Camera;
use crate::rar::effect_ids::EffectId;
use crate::rar::entities::Entity;
use crate::rar::entities::EntityConfiguration;
use crate::rar::entities::EntityData;
use crate::rar::entities::EntityType;
use crate::rar::layer_ids::LayerId;
use crate::rar::map::ObjectData;
use crate::rar::EntityUpdateContext;

const FPS: f32 = 25.0;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum PlayerState {
	WaitForStart,
	Idle,
	Backflip,
	Dying,
	Dead,
}

impl From<PlayerState> for &str {
	fn from(ps: PlayerState) -> Self {
		match ps {
			PlayerState::WaitForStart => "wait_for_start",
			PlayerState::Idle => "idle",
			PlayerState::Backflip => "backflip",
			PlayerState::Dying => "dying",
			PlayerState::Dead => "dead",
		}
	}
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum PlayerDirection {
	Left,
	Right,
}

impl From<PlayerDirection> for &str {
	fn from(ps: PlayerDirection) -> Self {
		match ps {
			PlayerDirection::Left => "left",
			PlayerDirection::Right => "right",
		}
	}
}

#[derive(Debug)]
pub struct EntityStateDirection {
	name:             String,
	//	template:         String,	-> moved into animated_texture
	animated_texture: AnimatedTexture,
}

impl EntityStateDirection {
	pub fn new(name: &str, template: &str, first_frame: u16, last_frame: u16, fps: f32) -> Self {
		let mut animated_texture = AnimatedTexture::new();
		animated_texture.setup(template, first_frame, last_frame, fps);
		Self {
			name: name.to_string(),
			//			template: template.to_string(),
			animated_texture,
		}
	}
}

#[derive(Debug)]
pub struct EntityState {
	name:       String,
	// -> moved into animated_texture
	/*
	first_frame: u16,
	last_frame:  u16,
	size:        [f32; 2],
	offset:      [f32; 2],
	*/
	directions: HashMap<String, EntityStateDirection>,
}

impl EntityState {
	pub fn new(
		name: &str,
		_first_frame: u16,
		_last_frame: u16,
		_size: &[f32; 2],
		_offset: &[f32; 2],
	) -> Self {
		Self {
			name:       name.to_string(),
			// -> moved into animated_texture
			/*
			first_frame,
			last_frame,
			size: size.clone(),
			offset: offset.clone(),
			*/
			directions: HashMap::new(),
		}
	}
	pub fn add_direction(&mut self, direction: EntityStateDirection) {
		self.directions.insert(direction.name.clone(), direction);
	}
}

#[derive(Debug)]
pub struct Player {
	name:                String,
	spawn_pos:           Vector2,
	pos:                 Vector2,
	old_pos:             Vector2,
	size:                Vector2,
	state:               PlayerState,
	direction:           PlayerDirection,
	speed:               Vector2,
	movement:            Vector2,
	time_since_dying:    f32,
	input_context_index: u8,
	entity_data:         EntityData,

	last_collision_line: Cell<Option<(Vector2, Vector2, Color)>>,
	states:              HashMap<String, EntityState>,

	// debug
	collision_history: VecDeque<Option<Cardinals>>,
}

impl Player {
	pub fn new() -> Self {
		Self {
			name:                "player".to_string(),
			spawn_pos:           Vector2::new(0.0, 0.0),
			pos:                 Vector2::zero(),
			old_pos:             Vector2::zero(),
			size:                Vector2::new(128.0, 128.0),
			state:               PlayerState::Dead,
			direction:           PlayerDirection::Right,
			speed:               Vector2::zero(),
			movement:            Vector2::zero(),
			time_since_dying:    f32::MAX,
			input_context_index: 0xff,
			entity_data:         EntityData::default(),

			last_collision_line: Cell::new(None),

			states: HashMap::new(),

			collision_history: VecDeque::new(),
		}
	}

	pub fn name(&self) -> &str {
		&self.name
	}

	pub fn is_alive(&self) -> bool {
		match self.state {
			PlayerState::Dead | PlayerState::Dying => false,
			PlayerState::WaitForStart | PlayerState::Idle | PlayerState::Backflip => true,
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
				self.pos.x += o * 200.0 - 100.0;
				self.direction = PlayerDirection::Right;
			},
			PlayerState::Idle => {},
			PlayerState::Backflip => {
				// :TODO: reset animation to frame 0
				self.state = state; // :HACK: so we get the correct state direction below
				if let Some(state_direction) = self.get_state_direction_mut() {
					println!("{:#?}", &state_direction);
					state_direction.animated_texture.set_autoloop(false);
					state_direction.animated_texture.set_current_frame(0);
				}
			},
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
		if let Some(pic) = euc.player_input_context(self.input_context_index) {
			if pic.is_left_pressed || pic.is_right_pressed {
				self.goto_state(PlayerState::Idle); // :TODO: start logic
			}
		}
	}

	fn update_idle(&mut self, euc: &mut EntityUpdateContext) {
		if let Some(pic) = euc.player_input_context(self.input_context_index) {
			const BACKFLIP_ENABLED: bool = !true;
			if BACKFLIP_ENABLED && pic.is_up_pressed {
				self.goto_state(PlayerState::Backflip);
			// self.speed = -100.0;
			// self.direction = PlayerDirection::Left;
			} else if pic.is_left_pressed {
				self.speed.x = -100.0;
				self.direction = PlayerDirection::Left;
			} else if pic.is_right_pressed {
				self.speed.x = 100.0;
				self.direction = PlayerDirection::Right;
			} else {
				self.speed.x = 0.0;
			}
			// :HACK:
			if pic.is_up_pressed {
				self.speed.y = 100.0;
			} else if pic.is_down_pressed {
				self.speed.y = -100.0;
			} else {
				self.speed.y -= 5.0;
			};
		}
	}

	fn update_backflip(&mut self, _euc: &mut EntityUpdateContext) {
		if let Some(state_direction) = self.get_state_direction_mut() {
			// println!("{:#?}", &state_direction );
			if state_direction.animated_texture.completed() {
				self.goto_state(PlayerState::Idle);
			}
		}
	}

	fn debug_colliders(&mut self, euc: &EntityUpdateContext) {
		let world = euc.world();
		//debug!("World {:?}", world);
		//list_objects_in_layer_for_class
		//let colliders = world.list_objects_in_layer_for_class( "Collider", "Collider" );
		let colliders = world.list_objects_in_layer("Collider");

		let start = &self.old_pos;
		let end = self.pos.clone();
		let l = start.sub(&end).length();
		let r = Rectangle::default()
			.with_size(&Vector2::new(12.0, 120.0))
			.with_center(&end);
		let pc = r.calculate_bounding_circle();
		//debug!("l: {}", l);
		let er = pc.radius() + l * 1.0;
		let pc = pc.with_radius(er);
		debug_renderer::debug_renderer_add_circle(pc.center(), pc.radius(), 5.0, &Color::white());
		debug_renderer::debug_renderer_add_rectangle(&r, 5.0, &Color::white());

		// :TODO: move away
		// y
		let vec = oml_game::DefaultTelemetry::get_f32("player.speed.y");
		for (i, sey) in vec.windows(2).enumerate() {
			let sy = sey[0];
			let ey = sey[1];
			let s = Vector2::new(i as f32 * 2.0 + 128.0, sy - 512.0);
			let e = Vector2::new((i + 1) as f32 * 2.0 + 128.0, ey - 512.0);

			debug_renderer::debug_renderer_add_line(&s, &e, 1.5, &Color::red());
		}

		// x
		let vec = oml_game::DefaultTelemetry::get_f32("player.speed.x");
		for (i, sey) in vec.windows(2).enumerate() {
			let sy = sey[0];
			let ey = sey[1];
			let s = Vector2::new(i as f32 * 2.0 + 128.0, sy - 512.0);
			let e = Vector2::new((i + 1) as f32 * 2.0 + 128.0, ey - 512.0);

			debug_renderer::debug_renderer_add_line(&s, &e, 1.5, &Color::blue());
		}

		// x
		let vec: Vec<&Option<Cardinals>> = self
			.collision_history
			.iter()
			.map(|c| c)
			.collect::<Vec<&Option<Cardinals>>>();
		for (i, cardinal) in vec.iter().enumerate() {
			if let Some(c) = &cardinal {
				let (col, o) = match c {
					Cardinals::Bottom => (Color::green(), -64.0),
					Cardinals::Left | Cardinals::Right => (Color::blue(), 64.0),
					_ => continue,
				};
				let s = Vector2::new((i as f32) * 2.0 + 128.0, 128.0 - 512.0 + o);
				let e = Vector2::new((i as f32) * 2.0 + 128.0, -128.0 - 512.0 + o);

				debug_renderer::debug_renderer_add_line(&s, &e, 1.5, &col);
			}
		}

		let mut collision_cardinal: Option<Cardinals> = None;
		for c in colliders {
			match c.data() {
				ObjectData::Rectangle {
					rect,
					bounding_circle,
				} => {
					let rect = rect.clone();
					//rect.offset(&offset);

					// if we have a bounding circle use that for a quick/cheap early out
					if let Some(bounding_circle) = bounding_circle {
						if pc.overlaps(&bounding_circle) {
							debug_renderer::debug_renderer_add_circle(
								bounding_circle.center(),
								bounding_circle.radius(),
								5.0,
								&Color::red(),
							);
						} else {
							debug_renderer::debug_renderer_add_circle(
								bounding_circle.center(),
								bounding_circle.radius(),
								5.0,
								&Color::blue(),
							);
							continue;
						}
					}

					debug_renderer::debug_renderer_add_rectangle(&rect, 5.0, &Color::red());

					if let Some(col) = rect.would_collide(&start, &end, &r) {
						debug!("Collision: {:?}", &col);
						let p = col.0 * 0.5;
						let full = end.sub(&start).scaled(p);
						let actual = start.add(&full);
						let r = r.clone().with_center(&actual);
						let l = match col.1 {
							Cardinals::Bottom => {
								self.pos = *r.center();
								self.pos.y += 1.0;
								self.speed.y = 0.0;
								let x0 = r.left();
								let x1 = r.right();
								let y = r.bottom();
								Some((Vector2::new(x0, y), Vector2::new(x1, y)))
							},
							Cardinals::Top => {
								let x0 = r.left();
								let x1 = r.right();
								let y = r.top();
								Some((Vector2::new(x0, y), Vector2::new(x1, y)))
							},
							Cardinals::Left => {
								self.pos = *r.center();
								self.pos.x += 1.0;
								self.speed.x = 0.0;
								let x = r.left();
								let y0 = r.bottom();
								let y1 = r.top();
								Some((Vector2::new(x, y0), Vector2::new(x, y1)))
							},
							Cardinals::Right => {
								let x = r.right();
								let y0 = r.bottom();
								let y1 = r.top();
								Some((Vector2::new(x, y0), Vector2::new(x, y1)))
							},
						};

						if let Some(l) = l {
							debug!("{:?}", &l);
							self.last_collision_line.set(Some((l.0, l.1, Color::red())));
							collision_cardinal = Some(col.1);
						}
					}

					//debug!("{:?}", &rect);
				},
				o => {
					debug!("Collider: {:?}", &o);
				},
			}
			//break;
		}

		self.collision_history.push_back(collision_cardinal);

		if let Some(l) = self.last_collision_line.get() {
			debug_renderer::debug_renderer_add_line(
				&l.0,
				&l.1,
				3.0,
				&Color::from_rgba(0.8, 0.6, 0.4, 1.0),
			);
		}

		//		debug!("Colliders {:?}", &colliders);
	}

	pub fn set_spawn_pos(&mut self, spawn_pos: &Vector2) {
		self.spawn_pos = *spawn_pos;
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

	pub fn set_input_context_index(&mut self, index: u8) {
		self.input_context_index = index;
	}

	fn add_state(&mut self, state: EntityState) {
		self.states.insert(state.name.clone(), state);
	}

	fn setup_from_configuration(&mut self, ec: &EntityConfiguration) {
		for (_sk, sv) in ec.states_iter() {
			let mut s = EntityState::new(
				sv.name(),
				sv.first_frame(),
				sv.last_frame(),
				sv.size(),
				sv.offset(),
			);

			for (_dk, dv) in sv.directions_iter() {
				let d = EntityStateDirection::new(
					dv.name(),
					dv.template(),
					sv.first_frame(),
					sv.last_frame(),
					FPS,
				);
				s.add_direction(d);
			}

			self.add_state(s);
		}
	}

	fn get_state_direction_mut(&mut self) -> Option<&mut EntityStateDirection> {
		let ps: &str = self.state.into();
		if let Some(state) = self.states.get_mut(ps) {
			let d: &str = self.direction.into();
			if let Some(state_direction) = state.directions.get_mut(d) {
				// println!("{:#?}", &state_direction );
				return Some(state_direction);
			}
		}
		None
	}
	fn get_state_direction(&mut self) -> Option<&EntityStateDirection> {
		let ps: &str = self.state.into();
		if let Some(state) = self.states.get(ps) {
			let d: &str = self.direction.into();
			if let Some(state_direction) = state.directions.get(d) {
				// println!("{:#?}", &state_direction );
				return Some(state_direction);
			}
		}
		None
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

	fn setup(&mut self, ec: &EntityConfiguration) {
		self.setup_from_configuration(&ec);
	}

	fn teardown(&mut self) {}

	fn update(&mut self, euc: &mut EntityUpdateContext) {
		// debug!("Player update time step: {}", euc.time_step() );
		if let Some(state_direction) = self.get_state_direction_mut() {
			// println!("{:#?}", &state_direction );
			state_direction.animated_texture.update(euc.time_step());
		}

		match self.state {
			PlayerState::WaitForStart => self.update_waiting_for_start(euc),
			PlayerState::Idle => self.update_idle(euc),
			PlayerState::Backflip => self.update_backflip(euc),
			PlayerState::Dying => self.goto_state(PlayerState::Dead),
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

	fn fixed_update(&mut self, euc: &EntityUpdateContext) {
		//		println!("Player: {:?}", &self);
		// :TODO: time step
		// debug!("Player fixed update time step: {}", euc.time_step() );

		self.old_pos = self.pos;

		self.movement.x = self.speed.x * euc.time_step() as f32;
		self.movement.y = self.speed.y * euc.time_step() as f32;
		self.pos = self.pos.add(&self.movement);

		self.debug_colliders(euc);
		/*
		debug!(
			"player delta pos y {} ({})",
			self.pos.y - self.old_pos.y,
			euc.time_step()
		);
		*/
		oml_game::DefaultTelemetry::trace_f32("player.speed.x", self.speed.x);
		oml_game::DefaultTelemetry::trace_f32("player.speed.y", self.speed.y);
		/*
				self.speed_history.push_back(self.speed);
				if self.speed_history.len() > 1000 {
					self.speed_history.pop_front();
				}
		*/
		if self.collision_history.len() > 1000 {
			self.collision_history.pop_front();
		}
	}

	fn render(&mut self, renderer: &mut Renderer, camera: &Camera) {
		if self.state == PlayerState::Dead {
			// dead means offscreen, nothing to be rendered
			return;
		}

		renderer.use_layer(LayerId::Player as u8);
		renderer.use_effect(EffectId::Textured as u16);

		if let Some(state_direction) = self.get_state_direction() {
			state_direction.animated_texture.r#use(renderer);
		}

		let pos = self.pos.add(&camera.offset());

		renderer.render_textured_quad(&pos, &self.size);
	}

	fn name(&self) -> &str {
		&self.name
	}

	fn entity_type(&self) -> EntityType {
		EntityType::Player
	}
}
