use std::cell::RefCell;
use std::rc::Rc;

//use oml_game::math::Vector2;
//use crate::fiiish::game::GameState;
use oml_game::renderer::debug_renderer::DebugRenderer;

use crate::rar::PlayerInputContext;
use crate::rar::World;

#[derive(Debug)]
pub struct EntityUpdateContext<'a> {
	time_step:             f64,
	player_input_contexts: Vec<PlayerInputContext>,
	world:                 &'a World,
	fixed_update_count:    u32,
	//	world_movement: Vector2,
	//	change_background_state: bool,
	//	game_state: GameState,
	pub debug_renderer:    Rc<Option<RefCell<DebugRenderer>>>,
}

static EMPTY_WORLD: World = World::new();

impl<'a> EntityUpdateContext<'a> {
	pub fn new() -> Self {
		Self {
			time_step:             0.0,
			player_input_contexts: Vec::new(),
			world:                 &EMPTY_WORLD,
			fixed_update_count:    0,
			//			world_movement: Vector2::zero(),
			//			change_background_state: false,
			//			game_state: GameState::None,
			debug_renderer:        Rc::new(None),
		}
	}

	pub fn player_input_context(&mut self, index: u8) -> Option<&mut PlayerInputContext> {
		if (index as usize) < self.player_input_contexts.len() {
			Some(&mut self.player_input_contexts[index as usize])
		} else {
			None
		}
	}

	pub fn add_player_input_context(&mut self, player_input_context: PlayerInputContext) {
		self.player_input_contexts.push(player_input_context);
	}

	pub fn time_step(&self) -> f64 {
		self.time_step
	}

	pub fn set_time_step(mut self, time_step: f64) -> Self {
		self.time_step = time_step;
		self
	}

	pub fn fixed_update_count(&self) -> u32 {
		self.fixed_update_count
	}

	pub fn with_fixed_update_count(mut self, fixed_update_count: u32) -> Self {
		self.fixed_update_count = fixed_update_count;
		self
	}

	/*
		pub fn world_movement(&self) -> &Vector2 {
			&self.world_movement
		}

		pub fn set_world_movement(&mut self, world_movement: &Vector2 ) {
			self.world_movement = *world_movement;
		}
	*/
	/*
		pub fn set_game_state(&mut self, game_state: &GameState ) {
			self.game_state = *game_state;
		}
	*/
	pub fn set_debug_renderer(&mut self, debug_renderer: &Rc<Option<RefCell<DebugRenderer>>>) {
		self.debug_renderer = Rc::clone(debug_renderer);
	}

	pub fn with_world(mut self, world: &'a World) -> Self {
		self.world = world;
		self
	}

	pub fn world(&self) -> &World {
		self.world
	}
	/*
		pub fn game_state( &self ) -> &GameState {
			&self.game_state
		}
	*/
	/*
		pub fn change_background_state( &self ) -> bool {
			self.change_background_state
		}
		pub fn enable_change_background_state( &mut self ) {
			self.change_background_state = true;
		}
	*/
}
