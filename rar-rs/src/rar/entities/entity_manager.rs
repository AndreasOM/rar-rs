use crate::rar::entities::Entity;

use std::collections::HashMap;

type EntityId = u32;

#[derive(Debug)]
pub struct EntityManager {
	next_id: EntityId,
	entities: HashMap< EntityId, Box<dyn Entity> >,
}

impl Default for EntityManager {
	fn default() -> Self {
		Self {
			next_id: 1,
			entities: HashMap::new(),
		}
	}
}
impl EntityManager {
	pub fn new() -> Self {
		Self {
			..Default::default()
		}
	}

	pub fn setup(&mut self) {}

	pub fn teardown(&mut self) {
		for (id,mut e) in self.entities.drain() {
			e.teardown();
		}
	}

	pub fn add(&mut self, entity: Box<dyn Entity>) -> EntityId {
		let id = self.next_id;
		self.next_id += 1; // :TODO: handle overflow ;)

		self.entities.insert( id, entity );
		id
	}

	pub fn get(&self, id: EntityId ) -> Option< &Box< dyn Entity > > {
		self.entities.get( &id )
	}

	pub fn get_as< T: 'static >(&self, id: EntityId ) -> Option< &T > {
		if let Some( e ) = self.entities.get( &id ) {
			// non noisy version e.as_any().downcast_ref::<T>()
			match e.as_any().downcast_ref::<T>() {
				Some(t) => Some(t),
				None => {
					eprintln!("{:?} isn't a {}!", &e, std::any::type_name::<T>() );
					None
				},
			}
		} else {
			None
		}
	}

	pub fn iter_mut(&mut self) -> std::collections::hash_map::ValuesMut<'_, u32, Box<(dyn Entity + 'static)>> { //std::slice::IterMut<'_, Box<(dyn Entity + 'static)>> {
		self.entities.values_mut()
	}

	pub fn remove_dead(&mut self) {
		// :TODO:
		/*
		for i in (0..self.entities.len()).rev() {
			if self.entities[i].is_dead() {
				//				println!("Cleaning dead {:?}", &self.entities[ i ] );
				self.entities.swap_remove(i);
			}
		}
		*/
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::rar::entities::background::Background;
	use crate::rar::entities::player::Player;

	use oml_game::math::Vector2;

	#[test]
	fn adding_and_getting_entities_works() -> anyhow::Result<()> {

		let mut em = EntityManager::default();

		let mut player1 = Player::new();
		player1.set_pos( &Vector2::new( 1.0, 1.0 ) );
		let id1 = em.add( Box::new( player1 ) );
		assert_eq!(id1, 1);

		let mut player2 = Player::new();
		player2.set_pos( &Vector2::new( 2.0, 2.0 ) );
		let id2 = em.add( Box::new( player2 ) );
		assert_eq!(id2, 2);

		let entity1 = em.get( id1 );
		assert_eq!(entity1.is_some(), true);

		assert_eq!( true, em.get_as::< Background >( id1 ).is_none() );	// wrong type

		let player1 = em.get_as::< Player >( id1 ).unwrap();	
		assert_eq!( Vector2::new( 1.0, 1.0 ), *player1.pos() );

		let player2 = em.get_as::< Player >( id2 ).unwrap();
		assert_eq!( Vector2::new( 2.0, 2.0 ), *player2.pos() );

		Ok(())
	}
}
