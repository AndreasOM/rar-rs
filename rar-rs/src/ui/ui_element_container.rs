use crate::ui::UiElementContainerData;
use std::cell::{Ref, RefCell, RefMut};
use std::collections::HashMap;
use std::rc::{Rc, Weak};
use std::str::FromStr;
use std::sync::mpsc::Sender;

use oml_game::math::Vector2;
use oml_game::renderer::debug_renderer::DebugRenderer;
use oml_game::renderer::Color;
use oml_game::system::System;
use serde::{Deserialize, Serialize};
use tracing::*;
use yaml_patch::Patch;

use crate::ui::UiElementFactory;
use crate::ui::{UiDebugConfig, UiDebugConfigMode};
use crate::ui::{
	UiElement, UiElementFadeData, UiElementFadeState, UiEvent, UiEventResponse, UiRenderer,
};

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
	pub fn from_config_asset(
		system: &mut System,
		ui_element_factory: &UiElementFactory,
		name: &str,
	) -> Option<Self> {
		let dfs = system.default_filesystem_mut();
		// try yaml
		let name_yaml = format!("{}.ui_config.yaml", &name);
		if dfs.exists(&name_yaml) {
			let mut f = dfs.open(&name_yaml);
			// :TODO: check is_valid ?
			let yaml = f.read_as_string();
			Some(Self::from_yaml(&ui_element_factory, &yaml))
		} else {
			// create fallback
			let mut container = Self::from_yaml(
				&ui_element_factory,
				"
type: UiImage
name: fallback
image: ui-button_confirm_danger
size: 64x64
fade:
  - out 0.0
  - in 1.0
children:
  - type: UiLabel
    tag: fallback_label
    size: 128x32
    text: Unable to load ui config from asset
    fade:
      - out 0.0
      - in 1.0
",
			);
			container.set_name(&format!("Fallback for {}", &name));
			container.find_child_by_tag_as_mut_element_then::<crate::ui::UiLabel>(
				"fallback_label",
				&|l| {
					l.set_text(&format!("UI config not found {}", &name));
				},
			);
			Some(container)
		}
	}

	pub fn from_yaml(ui_element_factory: &UiElementFactory, yaml: &str) -> Self {
		let value: serde_yaml::Value = serde_yaml::from_str(&yaml).unwrap();
		//		let config: UiElementContainerConfig = serde_yaml::from_str(&yaml).unwrap();
		//		let s: String = serde_yaml::from_value(val).unwrap();

		UiElementContainer::from_yaml_value(&ui_element_factory, value)
	}

	pub fn from_yaml_value(
		ui_element_factory: &UiElementFactory,
		yaml_value: serde_yaml::Value,
	) -> Self {
		let config: UiElementContainerConfig = serde_yaml::from_value(yaml_value.clone()).unwrap();

		let mut element = ui_element_factory
			.produce_element(&config.element_type)
			.unwrap_or_else(|| {
				error!(
					"Creating from yaml not supported for {}",
					&config.element_type
				);
				panic!();
			});
		/*
		let mut element: Box<dyn UiElement> = match config.element_type.as_ref() {
			"UiButton" => Box::new(crate::ui::UiButton::default()),
			"UiToggleButton" => Box::new(crate::ui::UiToggleButton::default()),
			"UiSpacer" => Box::new(crate::ui::UiSpacer::default()),
			"UiGridBox" => Box::new(crate::ui::UiGridBox::default()),
			"UiLabel" => Box::new(crate::ui::UiLabel::default()),
			"UiImage" => Box::new(crate::ui::UiImage::default()),
			o => {
				error!("Creating from yaml not supported for {}", &o);
				panic!();
			},
		};
		*/
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
		if let Some(fades) = &config.fade {
			for f in fades.iter() {
				let f: Vec<&str> = f.split(" ").collect();
				if f.len() == 2 {
					let duration = f32::from_str(f[1].trim()).unwrap_or(0.0);
					match f[0].trim() {
						"in" => {
							debug!("Fade in {}", duration);
							container = container.with_fade_in(duration);
						},
						"out" => {
							debug!("Fade out {}", duration);
							container = container.with_fade_out(duration);
						},
						o => {
							warn!("Unhandled fade mode {}", o);
						},
					}
				}
			}
		}
		config.children.map(|children| {
			for c in children.iter() {
				debug!("Child: {:?}", &c);
				let child_container =
					UiElementContainer::from_yaml_value(ui_element_factory, c.clone());
				container.add_child(child_container);
			}
		});
		//todo!();
		container
	}

	pub fn to_yaml_config_string(&self) -> String {
		let yaml = self.to_yaml_config();
		serde_yaml::to_string(&SortDataElement(&yaml)).unwrap_or("".to_string())
	}
	pub fn to_yaml_config(&self) -> serde_yaml::Value {
		/*
		struct UiElementContainerConfig {
			#[serde(rename = "type")]
			element_type: String,
			name:         Option<String>,
			tag:          Option<String>,
			//	children:     Option<Vec<UiElementContainerChildConfig>>,
			children:     Option<Vec<serde_yaml::Value>>,
			fade:         Option<Vec<String>>,
		}
		*/
		let children: Vec<serde_yaml::Value> = self
			.data
			.children
			.iter()
			.map(|c| c.borrow().to_yaml_config())
			.collect();
		//let children = serde_yaml::Value::Sequence( children );
		let mut c = UiElementContainerConfig {
			element_type: self.element.type_name().to_string(),
			name:         if self.data.name.is_empty() {
				None
			} else {
				Some(self.data.name.clone())
			},
			tag:          self.data.tag.clone(),
			children:     if children.is_empty() {
				None
			} else {
				Some(children)
			},
			fade:         None,
		};

		let element_yaml = self.element.to_yaml_config();

		//serde_yaml::to_value( element_yaml ).unwrap_or( serde_yaml::Value::Null )
		debug!("{:?}", &c);
		println!("{:?}", &element_yaml);

		let mut c_yaml = serde_yaml::to_value(c).unwrap_or(serde_yaml::Value::Null);
		UiElementContainer::merge_yaml(&mut c_yaml, element_yaml);
		/*
		c.patch_from_value( &element_yaml ).unwrap();
		c.patch_from_key_val( "element_type=UiGridBox____PATCHED" ).unwrap();
		c.patch_from_key_val( "padding=1234.56" ).unwrap();
		*/
		debug!("{:?}", &c_yaml);

		//serde_yaml::to_value( c ).unwrap_or( serde_yaml::Value::Null )
		c_yaml
		//serde_yaml::Value::Null
		/*
		- type: UiButton
		  size: 64x64
		  image: ui-button_back
		  name: Back
		  tag: back
		 */
	}
	fn merge_yaml(a: &mut serde_yaml::Value, b: serde_yaml::Value) {
		println!("Merging");
		match (a, b) {
			(a @ &mut serde_yaml::Value::Mapping(_), serde_yaml::Value::Mapping(b)) => {
				println!("Merging values");
				let a = a.as_mapping_mut().unwrap();
				for (k, v) in b {
					if v.is_sequence() && a.contains_key(&k) && a[&k].is_sequence() {
						let mut _b = a.get(&k).unwrap().as_sequence().unwrap().to_owned();
						_b.append(&mut v.as_sequence().unwrap().to_owned());
						a[&k] = serde_yaml::Value::from(_b);
						continue;
					}
					if !a.contains_key(&k) {
						println!("Adding {:?} = {:?}", &k, &v);
						a.insert(k.to_owned(), v.to_owned());
					} else {
						println!("Recursing {:?} = {:?}", &k, &v);
						Self::merge_yaml(&mut a[&k], v);
					}
				}
			},
			(_a, serde_yaml::Value::Null) => {
				println!("Ignoring NULL");
			},
			(a, b) => {
				println!("Replacing {:?} <= {:?}", &a, &b);

				*a = b
			},
		}
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
				"C  {} {}[{}] ({}): {}x{} @{},{} +({},{})",
				indent,
				&self.data.name,
				&self.data.tag().unwrap_or(&"".to_string()),
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

	pub fn refresh_tags( &mut self ) {
		self.data.refresh_tags();
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

	pub fn recalculate_size(&mut self) {
		self.element.recalculate_size(&mut self.data);
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

	pub fn data(&self) -> &UiElementContainerData {
		&self.data
	}
	pub fn data_mut(&mut self) -> &mut UiElementContainerData {
		&mut self.data
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
			UiEvent::MouseClick { pos, button } if self.data.is_visible() => {
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
			debug!("Found tag {}", &tag);
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
		f: &mut dyn FnMut(&mut UiElementContainer),
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
}

// :TODO: write custom serializer, to ensure 'correct' order
#[serde_with::skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
struct UiElementContainerConfig {
	#[serde(rename = "type")]
	element_type: String,
	name:         Option<String>,
	tag:          Option<String>,
	//	children:     Option<Vec<UiElementContainerChildConfig>>,
	children:     Option<Vec<serde_yaml::Value>>,
	fade:         Option<Vec<String>>,
}

fn sort_data_element<T: Serialize, S: serde::Serializer>(
	value: &T,
	serializer: S,
) -> Result<S::Ok, S::Error> {
	let value = serde_yaml::to_value(value).map_err(serde::ser::Error::custom)?;
	//eprintln!("Value: {:?}", &value);
	let value = match value {
		serde_yaml::Value::Mapping(mapping) => {
			//eprintln!("Mapping: {:?}", &mapping);
			let data_names = &[serde_yaml::Value::String("children".to_string())];
			let l = mapping.len();
			let mut new_mapping = serde_yaml::Mapping::with_capacity(l);
			let mut data_mapping = serde_yaml::Mapping::with_capacity(l);
			for (k, v) in mapping {
				// :TODO: recurse into sub structs aka 'v'
				if data_names.contains(&k) {
					data_mapping.insert(k, v); // remember data
				} else {
					new_mapping.insert(k, v);
				}
			}
			for (k, v) in data_mapping {
				new_mapping.insert(k, v); // append the remembered data
			}
			serde_yaml::Value::Mapping(new_mapping)
		},
		serde_yaml::Value::Sequence(sequence) => serde_yaml::Value::Sequence(sequence),
		o => o,
	};
	value.serialize(serializer)
}

#[derive(Serialize)]
struct SortDataElement<T: Serialize>(#[serde(serialize_with = "sort_data_element")] T);
#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn can_to_yaml() {
		let container = UiElementContainer::from_yaml(
			&UiElementFactory::default().with_standard_ui_elements(),
			"
type: UiImage
name: fallback
image: ui-button_confirm_danger
size: 64x64
fade:
  - out 0.0
  - in 1.0
children:
 - type: UiLabel
   tag: fallback_label
   size: 128x32
   text: Unable to load ui config from asset
   fade:
     - out 0.0
     - in 1.0
",
		);

		eprintln!("{:#?}", &container);

		let config = container.to_yaml_config();
		eprintln!("{:#?}", &config);
		let config_yaml =
			serde_yaml::to_string(&SortDataElement(&config)).unwrap_or("".to_string());
		eprintln!("{}", &config_yaml);
	}
}
