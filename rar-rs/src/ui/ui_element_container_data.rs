use std::collections::HashMap;

use oml_game::math::Vector2;
use tracing::*;

use crate::ui::UiElement;
use crate::ui::UiElementContainer;
use crate::ui::UiElementContainerHandle;
use crate::ui::UiElementFadeState;

#[derive(Debug, Default)]
pub struct UiElementContainerData {
	pub name:       String,
	pub tag:        Option<String>,
	pub pos:        Vector2,
	pub size:       Vector2,
	pub fade_state: UiElementFadeState,
	pub children:   Vec<UiElementContainerHandle>,
	tag_map:        HashMap<String, usize>,
}

impl UiElementContainerData {
	pub fn new() -> Self {
		Default::default()
	}
	pub fn name(&self) -> &str {
		&self.name
	}

	pub fn tag(&self) -> Option<&str> {
		self.tag.as_ref().map(|x| &**x)
	}

	pub fn tags(&self) -> Vec<String> {
		// :TODO: we probably could just use the tag map here
		let mut tags = Vec::new();
		self.tag.as_ref().map(|t| tags.push(t.to_owned()));
		for c in self.children.iter() {
			let mut ct = c.borrow().data().tags();
			tags.append(&mut ct);
		}
		tags
	}
	pub fn set_size(&mut self, size: &Vector2) {
		self.size = *size;
	}
	pub fn set_pos(&mut self, pos: &Vector2) {
		self.pos = *pos;
	}
	pub fn borrow_children(&self) -> &Vec<UiElementContainerHandle> {
		&self.children
	}
	pub fn borrow_children_mut(&mut self) -> &mut Vec<UiElementContainerHandle> {
		&mut self.children
	}
	pub fn fade_state(&self) -> &UiElementFadeState {
		&self.fade_state
	}
	pub fn get_fade_level(&self) -> f32 {
		let fs = self.fade_state;
		match fs {
			UiElementFadeState::FadedOut => 0.0,
			UiElementFadeState::FadedIn => 1.0,
			UiElementFadeState::FadingIn(d) => d.level,
			UiElementFadeState::FadingOut(d) => d.level,
		}
	}
	pub fn is_visible(&self) -> bool {
		let fs = self.fade_state;
		match fs {
			UiElementFadeState::FadedOut => false,
			UiElementFadeState::FadedIn => true,
			UiElementFadeState::FadingIn(_) => true,
			UiElementFadeState::FadingOut(_) => false,
		}
	}

	pub fn refresh_tags(&mut self) {
		//debug!("Refreshing tags for {}", self.name );
		self.tag_map = HashMap::new();
		for (p, child) in &mut self.children.iter_mut().enumerate() {
			//debug!("Refreshing tags for child [{}]{}", p, child.borrow().data().name );
			let mut child_mut = child.borrow_mut();
			let child_data = child_mut.data_mut();
			child_data.refresh_tags();
			let ct = child_data.tags();
			for tag in ct.iter() {
				if self.tag_map.get(tag).is_some() {
					warn!("Duplicated tag: {} -> {:#?}", &tag, &self.tag_map);
				//todo!(); // :TODO: panic? or ignore? -> allow, and implement correctly!
				} else {
					//let p = self.children.len();
					//let p = 999; // :TODO:
					self.tag_map.insert(tag.to_owned(), p);
				}
			}
		}
	}
	pub fn add_child(&mut self, child: UiElementContainer) -> UiElementContainerHandle {
		/*
		let ct = child.data().tags();
		for tag in ct.iter() {
			if self.tag_map.get(tag).is_some() {
				warn!("Duplicated tag: {} -> {:#?}", &tag, &self.tag_map);
			//todo!(); // :TODO: panic? or ignore? -> allow, and implement correctly!
			} else {
				let p = self.children.len();
				self.tag_map.insert(tag.to_owned(), p);
			}
		}
		*/
		let mut handle = UiElementContainerHandle::new(child);
		let mut handle2 = handle.clone();
		handle.borrow_mut().set_handle(&mut handle2);

		self.children.push(handle);
		self.refresh_tags();
		let last = self.children.len() - 1;
		self.children[last].clone()
	}

	pub fn add_child_element(
		&mut self,
		element: impl UiElement + 'static,
	) -> UiElementContainerHandle {
		self.add_child(UiElementContainer::new(Box::new(element)))
	}

	pub fn add_child_element_container(
		&mut self,
		element_container: UiElementContainer,
	) -> UiElementContainerHandle {
		self.add_child(element_container)
	}
	/*
		pub fn find_child_container_mut_then(
			&mut self,
			path: &[&str],
			f: &mut dyn FnMut(&mut UiElementContainer),
		) {
			if path.is_empty() {
				return;
			}
			let (head, tail) = path.split_at(1);
			let head = head[0];

			// find a child that matches
			for c in self.children.iter_mut() {
				let mut c = c.borrow_mut();
				if c.name() == head {
					if tail.is_empty() {
						// found -> run f with container
						f(&mut c);
					} else {
						// path matches so far, go deeper
						c.find_child_container_mut_then(&tail, f);
					}
				}
			}
		}
	*/
	pub fn find_child_container_by_tag_mut_then(
		&mut self,
		tag: &str,
		f: &mut dyn FnMut(&mut UiElementContainer),
	) -> bool {
		// lookup in tag_map
		let maybe_index = self.tag_map.get(tag);
		let r = maybe_index
			.map(|i| {
				let r = self.children[*i]
					.borrow_mut()
					.find_child_container_by_tag_mut_then(tag, f);

				//debug!("{} <- UiElementContainerData::find_child_container_by_tag_mut_then", r);
				r
			})
			.unwrap_or(false);
		//debug!("! {} <- UiElementContainerData::find_child_container_by_tag_mut_then", r);
		r
	}

	pub fn find_child_by_tag_as_mut_element_then<E: 'static>(
		&mut self,
		tag: &str,
		f: &dyn Fn(&mut E),
	) {
		// lookup in tag_map
		let maybe_index = self.tag_map.get(tag);
		debug!(
			"Index for tag {} in {} -> {:?}",
			tag, self.name, maybe_index
		);
		maybe_index.map(|i| {
			self.children[*i]
				.borrow_mut()
				.find_child_by_tag_as_mut_element_then(tag, f)
		});
	}
	/*
		pub fn find_child_mut_as_element_then<E: 'static>(
			&mut self,
			path: &[&str],
			f: &dyn Fn(&mut E),
		) {
			if let Some(mut c) = self.find_child_mut(path) {
				let mut c = c.borrow_mut();
				let c = c.borrow_element_mut();
				match c.as_any_mut().downcast_mut::<E>() {
					Some(e) => {
						f(e);
					},
					None => panic!(
						"{:?} isn't a {:?} at {:#?}!",
						&c,
						std::any::type_name::<E>(),
						&path
					),
				}
			} else {
				warn!(
					"Cannot find {:?} at path {:#?}",
					std::any::type_name::<E>(),
					&path
				);
			}
		}
	*/
	/*
		pub fn find_child_mut(&mut self, path: &[&str]) -> Option<UiElementContainerHandle> {
			if path.len() == 0 {
				// nothing left to check
				return None;
			}
			let (head, tail) = path.split_at(1);
			let head = head[0];

			if head == self.name() {
				if tail.len() == 0 {
					todo!("Is searching for yourself in yourself actually a valid use case?");
				} else {
					return self.find_child_mut(tail);
				}
			}

			for c in self.borrow_children_mut().iter_mut() {
				if let Some(r) = c.borrow_mut().find_child_mut(path) {
					return Some(r);
				}
			}
			None
		}
	*/
	pub fn dump_info(&self) {
		self.dump_info_internal(&"", &Vector2::zero(), 0);
	}
	pub fn dump_info_internal(&self, indent: &str, offset: &Vector2, depth: usize) {
		debug!("{:?}", &self.tag_map);
		let new_indent = format!("{}  ", indent);
		for c in self.borrow_children().iter() {
			//			let co = offset; //.add( c.pos() );
			let co = offset.add(c.borrow().pos());
			c.borrow()
				.dump_info_internal(&new_indent, &co, depth.saturating_sub(1));
		}
	}
}
