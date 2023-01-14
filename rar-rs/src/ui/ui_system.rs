use std::sync::mpsc::Sender;

use oml_game::math::Vector2;
use oml_game::renderer::debug_renderer::DebugRenderer;
use oml_game::renderer::Renderer;
use oml_game::system::System;
use tracing::*;

use crate::rar::effect_ids::EffectId;
use crate::rar::layer_ids::LayerId;
use crate::rar::AppUpdateContext;
use crate::ui::UiElement;
use crate::ui::UiElementContainer;
use crate::ui::UiElementFadeState;
use crate::ui::UiEvent;
use crate::ui::UiEventResponse;
use crate::ui::UiGravityBox;
use crate::ui::UiRenderer;

#[derive(Debug, Default)]
pub struct UiSystem {
	name:                  String,
	root:                  Option<UiElementContainer>,
	event_response_sender: Option<Sender<Box<dyn UiEventResponse>>>,
}

impl UiSystem {
	pub fn setup(
		&mut self,
		name: &str,
		_system: &mut System,
		event_response_sender: Sender<Box<dyn UiEventResponse>>,
	) -> anyhow::Result<()> {
		self.name = name.to_owned();
		self.event_response_sender = Some(event_response_sender);
		let root = UiGravityBox::new()
			.with_padding(16.0)
			.containerize()
			.with_name(name);
		self.root = Some(root);
		Ok(())
	}

	pub fn teardown(&mut self) {
		if let Some(_ers) = self.event_response_sender.take() {}
		if let Some(_root) = self.root.take() {}
	}

	pub fn add_child(&mut self, gravity: &Vector2, child: UiElementContainer) {
		if let Some(root) = &mut self.root {
			{
				let root = root.borrow_element_mut();
				match root.as_any_mut().downcast_mut::<UiGravityBox>() {
					Some(root) => {
						root.set_gravity(gravity);
					},
					None => panic!("root for {} is not a UiGravityBox", &self.name),
				};
			}
			root.add_child(child);
		}
	}

	pub fn toggle_child_fade_by_tag(&mut self, tag: &str) {
		if let Some(root) = &mut self.root {
			let found = root.find_child_container_by_tag_mut_then(tag, &mut |dialog| match dialog
				.fade_state()
			{
				UiElementFadeState::FadedOut | UiElementFadeState::FadingOut(_) => {
					dialog.fade_in(1.0);
				},
				UiElementFadeState::FadedIn | UiElementFadeState::FadingIn(_) => {
					dialog.fade_out(1.0);
				},
			});
			if !found {
				warn!("Could toggle child for tag {}", &tag);
			}
		}
	}
	pub fn fade_out_child_by_tag(&mut self, tag: &str, duration: f32) {
		if let Some(root) = &mut self.root {
			let found = root.find_child_container_by_tag_mut_then(tag, &mut |child| {
				child.fade_out(duration);
			});
			if !found {
				warn!("Could fade out child for tag {}", &tag);
			}
		}
	}
	pub fn fade_in_child_by_tag(&mut self, tag: &str, duration: f32) {
		if let Some(root) = &mut self.root {
			let found = root.find_child_container_by_tag_mut_then(tag, &mut |child| {
				child.fade_in(duration);
			});
			if !found {
				warn!("Could fade in child for tag {}", &tag);
			}
		}
	}
	/*
		pub fn toggle_child_fade(&mut self, path: &[&str]) -> bool {
			let mut was_on = false;
			if let Some(root) = &mut self.root {
				root.find_child_container_mut_then(path, &mut |dialog| match dialog.fade_state() {
					UiElementFadeState::FadedOut | UiElementFadeState::FadingOut(_) => {
						dialog.fade_in(1.0);
						was_on = false
					},
					UiElementFadeState::FadedIn | UiElementFadeState::FadingIn(_) => {
						dialog.fade_out(1.0);
						was_on = true;
					},
				});
			}
			was_on
		}
	*/
	pub fn set_size(&mut self, size: &Vector2) {
		// :TODO-UI: should probably use parent_size_changed instead
		if let Some(root) = &mut self.root {
			root.parent_size_changed(size);
		}
		self.layout();
		self.dump_info();
	}

	pub fn layout(&mut self) {
		if let Some(root) = &mut self.root {
			root.layout(&Vector2::zero());
		}
	}

	pub fn dump_info(&self) {
		if let Some(root) = &self.root {
			root.dump_info();
		}
	}

	pub fn update(&mut self, auc: &mut AppUpdateContext) {
		if let Some(root) = &mut self.root {
			if let Some(wuc) = auc.wuc() {
				if wuc.was_mouse_button_pressed(0) {
					let cp = auc.cursor_pos();
					//debug!("Left Mouse Button was pressed @ {}, {}", cp.x, cp.y);
					//debug!("{:?}", &root);
					let ev = UiEvent::MouseClick {
						pos:    *cp,
						button: 0,
					};
					if let Some(event_response_sender) = &mut self.event_response_sender {
						debug!("{:?}", &root);
						if let Some(ev) = root.handle_ui_event(&ev, &event_response_sender) {
							debug!("Click handled");
							//let _ =
							event_response_sender.send(ev).unwrap();
						} else {
							//root.dump_info( "", &Vector2::zero() );
						}
					}
				}

				root.update(wuc.time_step());
			}
		}
	}
	pub fn render(&mut self, renderer: &mut Renderer) {
		if let Some(root) = &mut self.root {
			// :CHEAT: ???
			renderer.use_layer(LayerId::Ui as u8);

			let mut ui_renderer = UiRenderer::new(
				renderer,
				EffectId::ColoredTextured as u16,
				EffectId::Colored as u16,
				EffectId::FontColored as u16,
				LayerId::Ui as u8,
				LayerId::UiFront as u8,
			);
			root.render(&mut ui_renderer);
		}
	}

	pub fn render_debug(&mut self, debug_renderer: &mut DebugRenderer) {
		if let Some(root) = &mut self.root {
			root.render_debug(debug_renderer, &Vector2::zero(), 0);
		}
	}
}
