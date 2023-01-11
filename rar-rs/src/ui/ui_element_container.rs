use std::cell::{Ref, RefCell, RefMut};
use std::collections::HashMap;
use std::rc::{Rc, Weak};
use std::sync::mpsc::Sender;

use oml_game::math::Vector2;
use oml_game::renderer::debug_renderer::DebugRenderer;
use oml_game::renderer::Color;
use serde::Deserialize;
use tracing::*;

use crate::ui::{UiDebugConfig, UiDebugConfigMode};
use crate::ui::{
	UiElement, UiElementFadeData, UiElementFadeState, UiEvent, UiEventResponse, UiRenderer,
};

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

	pub fn tags(&self) -> Vec<String> {
		// :TODO: we probably could just use the tag map here
		let mut tags = Vec::new();
		self.tag.as_ref().map(|t| tags.push(t.to_owned()));
		for c in self.children.iter() {
			let mut ct = c.borrow().data.tags();
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

	pub fn add_child(&mut self, child: UiElementContainer) -> UiElementContainerHandle {
		let ct = child.data.tags();
		for tag in ct.iter() {
			if self.tag_map.get(tag).is_some() {
				warn!("Duplicated tag: {} -> {:#?}", &tag, &self.tag_map);
				todo!(); // :TODO: panic? or ignore?
			} else {
				let p = self.children.len();
				self.tag_map.insert(tag.to_owned(), p);
			}
		}
		let mut handle = UiElementContainerHandle::new(child);
		let mut handle2 = handle.clone();
		handle.borrow_mut().set_handle(&mut handle2);

		self.children.push(handle);
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
		f: &dyn Fn(&mut UiElementContainer),
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

#[derive(Debug, Clone)]
pub struct UiElementContainerHandleWeak {
	weak_handle: Weak<RefCell<UiElementContainer>>,
}

impl UiElementContainerHandleWeak {
	pub fn new(handle: Weak<RefCell<UiElementContainer>>) -> Self {
		Self {
			weak_handle: handle,
		}
	}
	pub fn upgrade(&mut self) -> UiElementContainerHandle {
		UiElementContainerHandle::upgrade(&mut self.weak_handle)
	}
}
#[derive(Debug, Clone)]
pub struct UiElementContainerHandle {
	container: Rc<RefCell<UiElementContainer>>,
}

impl UiElementContainerHandle {
	pub fn new(container: UiElementContainer) -> Self {
		Self {
			container: Rc::new(RefCell::new(container)),
		}
	}

	pub fn upgrade(handle: &mut Weak<RefCell<UiElementContainer>>) -> Self {
		Self {
			container: handle.upgrade().unwrap(),
		}
	}

	pub fn borrow(&self) -> Ref<UiElementContainer> {
		self.container.borrow()
	}
	pub fn borrow_mut(&mut self) -> RefMut<UiElementContainer> {
		self.container.borrow_mut()
	}

	pub fn downgrade(&mut self) -> UiElementContainerHandleWeak {
		UiElementContainerHandleWeak::new(Rc::downgrade(&self.container))
	}
}
/*
impl Copy for UiElementContainerHandle {

}
*/

#[derive(Debug)]
pub struct UiElementContainer {
	element: Box<dyn UiElement>,
	data:    UiElementContainerData,
	handle:  Option<UiElementContainerHandleWeak>,
}

impl UiElementContainer {
	pub fn new(mut element: Box<dyn UiElement>) -> Self {
		let mut data = UiElementContainerData::new();
		if let Some(size) = element.preferred_size() {
			//			println!("{:?} has a preferred size of {:?}", &element, &size );
			data.set_size(size);
		}
		element.setup_within_container(&mut data);
		Self {
			element,
			data,
			handle: None,
		}
	}
	pub fn from_yaml(yaml: &str) -> Self {
		let value: serde_yaml::Value = serde_yaml::from_str(&yaml).unwrap();
		//		let config: UiElementContainerConfig = serde_yaml::from_str(&yaml).unwrap();
		//		let s: String = serde_yaml::from_value(val).unwrap();
		UiElementContainer::from_yaml_value(value)
	}

	pub fn from_yaml_value(yaml_value: serde_yaml::Value) -> Self {
		let config: UiElementContainerConfig = serde_yaml::from_value(yaml_value.clone()).unwrap();

		let mut element: Box<dyn UiElement> = match config.element_type.as_ref() {
			"UiButton" => Box::new(crate::ui::UiButton::default()),
			"UiToggleButton" => Box::new(crate::ui::UiToggleButton::default()),
			"UiSpacer" => Box::new(crate::ui::UiSpacer::default()),
			"UiGridBox" => Box::new(crate::ui::UiGridBox::default()),
			o => {
				error!("Creating from yaml not supported for {}", &o);
				panic!();
			},
		};
		element.configure_from_yaml_value(yaml_value.clone());
		let mut container = UiElementContainer::new(element);
		/* other option, not finally decided yet
				let mut container = match config.element_type.as_ref() {
					"UiButton" => crate::ui::UiButton::from_yaml(yaml).containerize(),
					"UiSpacer" => crate::ui::UiSpacer::from_yaml(yaml).containerize(),
					//"UiGridBox" => crate::ui::UiGridBox::from_yaml(yaml).containerize(),
					"UiGridBox" => { let mut e = crate::ui::UiGridBox::default(); e.configure_from_yaml( yaml ); e.containerize() },
					o => {
						error!("Creating from yaml not supported for {}", &o);
						panic!();
					},
				};
		*/
		if let Some(tag) = &config.tag {
			container = container.with_tag(tag);
		}
		if let Some(name) = &config.name {
			container = container.with_name(name);
		}
		config.children.map(|children| {
			for c in children.iter() {
				debug!("Child: {:?}", &c);
				let child_container = UiElementContainer::from_yaml_value(c.clone());
				container.add_child(child_container);
			}
		});
		//todo!();
		container
	}

	pub fn new_from_element(element: impl UiElement + 'static) -> Self {
		UiElementContainer::new(Box::new(element))
	}

	pub fn set_handle(&mut self, handle: &mut UiElementContainerHandle) {
		self.handle = Some(handle.downgrade());
	}

	pub fn update(&mut self, time_step: f64) {
		self.element.update(&mut self.data, time_step);
		self.update_fade_state(time_step);
		for c in self.data.children.iter_mut() {
			c.borrow_mut().update(time_step);
		}
	}

	pub fn render(&self, ui_renderer: &mut UiRenderer) {
		self.element.render(&self.data, ui_renderer);
		self.render_children(ui_renderer);
	}

	pub fn render_children(&self, ui_renderer: &mut UiRenderer) {
		if self.data.fade_state != UiElementFadeState::FadedOut {
			ui_renderer.push_translation(&self.data.pos);
			let l = self.get_fade_level();
			ui_renderer.push_opacity(l);
			for c in self.data.children.iter() {
				c.borrow().render(ui_renderer);
			}
			ui_renderer.pop_opacity();
			ui_renderer.pop_transform();
		}
	}

	pub fn fade_state(&self) -> &UiElementFadeState {
		&self.data.fade_state
	}
	pub fn set_fade_state(&mut self, fade_state: &UiElementFadeState) {
		self.data.fade_state = *fade_state;
	}

	pub fn with_fade_in(mut self, duration: f32) -> Self {
		self.fade_in(duration);
		self
	}
	pub fn fade_in(&mut self, duration: f32) {
		let fs = self.fade_state();
		if duration == 0.0 {
			self.set_fade_state(&UiElementFadeState::FadedIn);
		} else {
			let speed = 1.0 / duration;
			match fs {
				UiElementFadeState::FadedIn => (),
				UiElementFadeState::FadedOut => {
					let fs = UiElementFadeState::FadingIn(UiElementFadeData { level: 0.0, speed });
					self.data.fade_state = fs;
				},
				UiElementFadeState::FadingIn(d) => {
					let fs = UiElementFadeState::FadingIn(UiElementFadeData {
						level: d.level,
						speed,
					});
					self.data.fade_state = fs;
				},
				UiElementFadeState::FadingOut(d) => {
					let fs = UiElementFadeState::FadingIn(UiElementFadeData {
						level: d.level,
						speed,
					});
					self.data.fade_state = fs;
				},
			}
		}
	}
	pub fn with_fade_out(mut self, duration: f32) -> Self {
		self.fade_out(duration);
		self
	}
	pub fn fade_out(&mut self, duration: f32) {
		let fs = self.fade_state();
		if duration == 0.0 {
			self.set_fade_state(&UiElementFadeState::FadedOut);
		} else {
			let speed = 1.0 / duration;
			match fs {
				UiElementFadeState::FadedOut => (),
				UiElementFadeState::FadedIn => {
					let fs = UiElementFadeState::FadingOut(UiElementFadeData { level: 1.0, speed });
					self.data.fade_state = fs;
				},
				UiElementFadeState::FadingIn(d) => {
					let fs = UiElementFadeState::FadingOut(UiElementFadeData {
						level: d.level,
						speed,
					});
					self.data.fade_state = fs;
				},
				UiElementFadeState::FadingOut(d) => {
					let fs = UiElementFadeState::FadingOut(UiElementFadeData {
						level: d.level,
						speed,
					});
					self.data.fade_state = fs;
				},
			}
		}
	}
	pub fn toggle_fade(&mut self, duration: f32) {
		let fs = self.fade_state();
		match fs {
			UiElementFadeState::FadedOut | UiElementFadeState::FadingOut(_) => {
				self.fade_in(duration);
			},
			UiElementFadeState::FadedIn | UiElementFadeState::FadingIn(_) => {
				self.fade_out(duration);
			},
		}
	}
	fn update_fade_state(&mut self, time_step: f64) {
		let fs = self.data.fade_state;
		match fs {
			UiElementFadeState::FadedOut => (),
			UiElementFadeState::FadedIn => (),
			UiElementFadeState::FadingIn(d) => {
				let new_level = d.level + d.speed * time_step as f32;
				if new_level < 1.0 {
					let fs = UiElementFadeState::FadingIn(UiElementFadeData {
						level: new_level,
						speed: d.speed,
					});
					self.data.fade_state = fs;
				} else {
					self.data.fade_state = UiElementFadeState::FadedIn;
				}
			},
			UiElementFadeState::FadingOut(d) => {
				let new_level = d.level - d.speed * time_step as f32;
				if new_level > 0.0 {
					let fs = UiElementFadeState::FadingOut(UiElementFadeData {
						level: new_level,
						speed: d.speed,
					});
					self.data.fade_state = fs;
				} else {
					self.data.fade_state = UiElementFadeState::FadedOut;
				}
			},
		}
	}

	pub fn get_fade_level(&self) -> f32 {
		self.data.get_fade_level()
	}

	pub fn render_debug(&self, debug_renderer: &mut DebugRenderer, offset: &Vector2, depth: usize) {
		if *self.fade_state() == UiElementFadeState::FadedOut {
			return;
		}

		let mut depth = depth;
		UiDebugConfig::read_then(&mut |ui_debug_config| match ui_debug_config.mode() {
			UiDebugConfigMode::All => {
				depth = 1;
			},
			UiDebugConfigMode::Selected => {
				if let Some(d) = ui_debug_config.is_selected(self.name()) {
					depth = d + 1;
				}
			},
			_ => {},
		});

		if depth > 0 {
			self.element
				.render_debug(&self.data, debug_renderer, offset);
		}
		for c in self.data.borrow_children().iter() {
			let co = offset.add(c.borrow().pos());
			c.borrow()
				.render_debug(debug_renderer, &co, depth.saturating_sub(1));
		}
		if depth > 0 {
			debug_renderer.add_line(
				&Vector2::zero(),
				&Vector2::zero().add(&offset),
				3.0,
				&Color::from_rgba(0.5, 0.5, 0.5, 0.8),
			);
			let hs = self.size().scaled(0.5);
			let bl = offset.sub(&hs);
			let tr = offset.add(&hs);
			let tl = Vector2::new(bl.x, tr.y);
			let br = Vector2::new(tr.x, bl.y);
			let color = Color::from_rgba(0.2, 0.9, 0.2, 0.3);
			debug_renderer.add_line(&tl, &bl, 3.0, &color);
			debug_renderer.add_line(&bl, &br, 3.0, &color);
			debug_renderer.add_line(&br, &tr, 3.0, &color);
			debug_renderer.add_line(&tr, &tl, 3.0, &color);
			/*
						let color = Color::from_rgba(0.2, 0.9, 0.2, 0.8);
						debug_renderer.add_text(
							&tr,
							&format!("{}/{} - {}", offset.x, offset.y, self.name()),
							20.0,
							2.0,
							&color,
						)
			*/
		}
	}

	pub fn dump_info(&self) {
		self.dump_info_internal(&"", &Vector2::zero(), 0);
	}
	pub fn dump_info_internal(&self, indent: &str, offset: &Vector2, depth: usize) {
		let mut depth = depth;
		UiDebugConfig::read_then(&mut |ui_debug_config| match ui_debug_config.mode() {
			UiDebugConfigMode::All => {
				depth = 1;
			},
			UiDebugConfigMode::Selected => {
				if let Some(d) = ui_debug_config.is_selected(self.name()) {
					depth = d + 1;
				}
			},
			_ => {},
		});

		if depth > 0 {
			println!(
				"C  {} {} ({}): {}x{} @{},{} +({},{})",
				indent,
				&self.data.name,
				self.element.type_name(),
				self.size().x,
				self.size().y,
				self.pos().x,
				self.pos().y,
				offset.x,
				offset.y,
			);
		}
		let new_indent = format!("{}  ", indent);
		for c in self.data.borrow_children().iter() {
			//			let co = offset; //.add( c.pos() );
			let co = offset.add(c.borrow().pos());
			c.borrow()
				.dump_info_internal(&new_indent, &co, depth.saturating_sub(1));
		}
	}

	pub fn borrow_element(&self) -> &Box<dyn UiElement> {
		&self.element
	}
	pub fn borrow_element_mut(&mut self) -> &mut Box<dyn UiElement> {
		&mut self.element
	}

	pub fn borrow_children(&self) -> &Vec<UiElementContainerHandle> {
		&self.data.children
	}

	pub fn borrow_children_mut(&mut self) -> &mut Vec<UiElementContainerHandle> {
		&mut self.data.children
	}

	pub fn add_child(&mut self, mut child: UiElementContainer) -> &mut UiElementContainerHandle {
		self.element.add_child(&mut child.data);
		//		self.data.children.push( child );
		self.data.add_child(child);
		self.element.recalculate_size(&mut self.data);
		let last = self.data.children.len() - 1;
		&mut self.data.children[last]
	}

	pub fn add_child_element(
		&mut self,
		element: impl UiElement + 'static,
	) -> &mut UiElementContainerHandle {
		self.add_child(UiElementContainer::new(Box::new(element)))
	}

	pub fn add_child_element_container(
		&mut self,
		element_container: UiElementContainer,
	) -> &mut UiElementContainerHandle {
		self.add_child(element_container)
	}

	pub fn with_child_element_containers(
		mut self,
		child_containers: Vec<UiElementContainer>,
	) -> Self {
		for cc in child_containers {
			self.add_child(cc);
		}

		self
	}

	pub fn parent_size_changed(&mut self, parent_size: &Vector2) {
		self.element
			.parent_size_changed(&mut self.data, parent_size);
	}

	pub fn layout(&mut self, pos: &Vector2) {
		/*
		debug!(
			"Container layout for {} -> {}, {}",
			&self.data.name, pos.x, pos.y
		);
		*/
		self.data.pos = *pos;
		self.element.layout(&mut self.data, pos);
	}

	pub fn size(&self) -> &Vector2 {
		&self.data.size
	}
	pub fn set_size(&mut self, size: &Vector2) {
		//		self.element.set_size( size );
		self.data.size = *size;
	}
	pub fn with_size(mut self, size: &Vector2) -> Self {
		self.data.size = *size;
		self
	}

	pub fn name(&self) -> &str {
		&self.data.name
	}
	pub fn set_name(&mut self, name: &str) {
		self.data.name = name.to_owned();
	}

	pub fn with_name(mut self, name: &str) -> Self {
		self.set_name(name);
		self
	}

	pub fn tag(&self) -> Option<&str> {
		self.data.tag.as_ref().map(|x| &**x)
	}
	pub fn with_tag(mut self, tag: &str) -> Self {
		self.data.tag = Some(tag.to_owned());
		self
	}

	pub fn pos(&self) -> &Vector2 {
		&self.data.pos
	}
	pub fn set_pos(&mut self, pos: &Vector2) {
		self.data.pos = *pos;
	}

	fn handle_mouse_click(
		&mut self,
		pos: &Vector2,
		button: u8,
		event: &UiEvent,
		event_sender: &Sender<Box<dyn UiEventResponse>>,
	) -> Option<Box<dyn UiEventResponse>> {
		let pos = pos.sub(self.pos());
		if self.is_hit_by(&pos) {
			//debug!( "Hit with {} children", self.borrow_base_mut().children.len() );
			//debug!("Hit {:?}", &self);
			debug!("Hit {:?} -> {}", &pos, self.name());
			self.dump_info_internal("", &Vector2::zero(), usize::MAX);
			for c in self.data.borrow_children_mut().iter_mut() {
				let mut c = c.borrow_mut();
				let cpos = pos.sub(c.pos());
				//						let pos = *pos;
				//						println!("New pos: {},{} (child @ {}, {} -> {}, {})", pos.x, pos.y , c.pos().x, c.pos().y, cpos.x, cpos.y );
				if c.is_hit_by(&cpos) {
					println!("Child is hit");
					let ev = UiEvent::MouseClick {
						pos,
						button: button,
					};
					if let Some(r) = c.handle_ui_event(&ev, event_sender) {
						//return self.element.handle_ui_event_response(r);
						return Some(r);
					}
				} else {
					debug!("Child >{}< NOT hit ({:?})", &c.name(), &c.size());
				}
			}
			// no child hit, so try give to our element
			self.element
				.handle_ui_event(&mut self.data, &event, event_sender)
		} else {
			debug!("Not hit: {:?}", &self);
			None
		}
	}

	pub fn handle_ui_event(
		&mut self,
		event: &UiEvent,
		event_sender: &Sender<Box<dyn UiEventResponse>>,
	) -> Option<Box<dyn UiEventResponse>> {
		match event {
			UiEvent::MouseClick { pos, button } => {
				if let Some(r) = self.handle_mouse_click(pos, *button, event, event_sender) {
					// self.element.handle_ui_event_response(r)
					self.handle_ui_event_response(r)
				} else {
					None
				}
			},
			#[allow(unreachable_patterns)]
			_ => None,
		}
	}

	fn handle_ui_event_response(
		&mut self,
		response: Box<dyn UiEventResponse>,
	) -> Option<Box<dyn UiEventResponse>> {
		self.element
			.handle_ui_event_response(&mut self.data, response)
	}

	// local coordinates!
	fn is_hit_by(&self, pos: &Vector2) -> bool {
		let hs = self.data.size.scaled(0.5);
		let bl = Vector2::zero().sub(&hs);
		let tr = Vector2::zero().add(&hs);
		pos.x >= bl.x && pos.y >= bl.y && pos.x <= tr.x && pos.y <= tr.y
	}

	pub fn find_child_by_tag_as_mut_element_then<E: 'static>(
		&mut self,
		tag: &str,
		f: &dyn Fn(&mut E),
	) {
		if self.data.tag == Some(tag.to_string()) {
			let c = &mut self.element;
			match c.as_any_mut().downcast_mut::<E>() {
				Some(e) => {
					f(e);
				},
				None => panic!(
					"{:?} isn't a {:?} with tag {:#?}!",
					&c,
					std::any::type_name::<E>(),
					&tag,
				),
			}
		} else {
			self.data.find_child_by_tag_as_mut_element_then(tag, f);
		}
	}

	pub fn find_child_container_by_tag_mut_then(
		&mut self,
		tag: &str,
		f: &dyn Fn(&mut UiElementContainer),
	) -> bool {
		if self.data.tag == Some(tag.to_string()) {
			f(self);
			//debug!("true");
			true
		} else {
			let r = self.data.find_child_container_by_tag_mut_then(tag, f);
			//debug!("{}", &r);
			r
		}
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
			for c in self.data.borrow_children_mut().iter_mut() {
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
	/*
	pub fn find_child_mut(&mut self, path: &[&str]) -> Option<UiElementContainerHandle> {
		if path.len() == 0 {
			// nothing left to check
			return None;
		}
		let (head, tail) = path.split_at(1);
		let head = head[0];

		//		println!("Checking {} for {}, {:?}", self.name(), head, tail );

		if head == self.name() {
			if tail.len() == 0 {
				//				println!("Found {}!", &head );
				//				return Some( &mut UiElementContainerHandle::new( *self ) );
				if let Some(handle) = &mut self.handle {
					return Some(handle.upgrade());
				} else {
					println!("Found {}, but it doesn't have a handle!", &head);
					return None;
				}
			} else {
				//				println!("Found {} ... {:?}", &head, &tail );
				return self.find_child_mut(tail);
			}
		}

		//		println!("Checking {} children for {}, {:?}", self.data.borrow_children().len(), head, tail );

		for c in self.data.borrow_children_mut().iter_mut() {
			if let Some(r) = c.borrow_mut().find_child_mut(path) {
				return Some(r);
			}
		}
		None
	}
	*/
}

#[derive(Debug, Deserialize)]
struct UiElementContainerConfig {
	#[serde(rename = "type")]
	element_type: String,
	name:         Option<String>,
	tag:          Option<String>,
	//	children:     Option<Vec<UiElementContainerChildConfig>>,
	children:     Option<Vec<serde_yaml::Value>>,
}
/*
use std::fmt;

#[derive(Debug, Default)]
struct UiElementContainerChildConfig {
	entries: Vec<String>,
}

impl UiElementContainerChildConfig {
	pub fn add(&mut self, v: &str) {
		self.entries.push(v.to_owned())
	}
}

struct UiElementContainerChildConfigVisitor {}

impl UiElementContainerChildConfigVisitor {
	fn new() -> Self {
		UiElementContainerChildConfigVisitor {}
	}
}

impl<'de> Visitor<'de> for UiElementContainerChildConfigVisitor {
	type Value = UiElementContainerChildConfig;

	fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
		formatter.write_str("a very special map")
	}
	fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
	where
		A: MapAccess<'de>,
	{
		let mut my_map = UiElementContainerChildConfig::default();
		while let Some((a, IgnoredAny)) = map.next_entry::<IgnoredAny, IgnoredAny>()? {
			debug!("{:?}", a);
			// would be lovely to access the underlying strings here :(
			my_map.add("...");
		}
		Ok(my_map)
	}
	/*
	fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
	where
		E: Error,
	{
		debug!("{}", v);
		let mut my_map = MyMap::default();
		Ok(my_map)
	}
	*/
}

// This is the trait that informs Serde how to deserialize MyMap.
impl<'de> Deserialize<'de> for UiElementContainerChildConfig {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: Deserializer<'de>,
	{
		// Instantiate our Visitor and ask the Deserializer to drive
		// it over the input data, resulting in an instance of UiElementContainerChildConfig.
		deserializer.deserialize_map(UiElementContainerChildConfigVisitor::new())
	}
}
*/
