use crate::rar::entities::Entity;

#[derive(Debug, Default)]
pub struct EntityManager {
	entities: Vec<Box<dyn Entity>>,
}

impl EntityManager {
	pub fn new() -> Self {
		Self {
			entities: Vec::new(),
		}
	}

	pub fn setup(&mut self) {}

	pub fn teardown(&mut self) {
		while let Some(mut e) = self.entities.pop() {
			e.teardown();
		}
	}

	pub fn add(&mut self, entity: Box<dyn Entity>) {
		self.entities.push(entity);
	}

	pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, Box<(dyn Entity + 'static)>> {
		self.entities.iter_mut()
	}

	pub fn remove_dead(&mut self) {
		//		let ni = self.entities.len();
		for i in (0..self.entities.len()).rev() {
			if self.entities[i].is_dead() {
				//				println!("Cleaning dead {:?}", &self.entities[ i ] );
				self.entities.swap_remove(i);
			}
		}
		//		let ri = ni - self.entities.len();
		//		if ri > 0 {
		//			println!("Removed {} entities. Now have {}.", ri, self.entities.len() );
		//		}
	}
}
