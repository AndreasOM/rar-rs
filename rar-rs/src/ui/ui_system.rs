use std::sync::mpsc::Sender;

use oml_game::math::Vector2;
use oml_game::renderer::debug_renderer::DebugRenderer;
use oml_game::renderer::Renderer;
use oml_game::system::System;
use tracing::*;

use crate::rar::effect_ids::EffectId;
use crate::rar::layer_ids::LayerId;
use crate::rar::AppUpdateContext;
use crate::ui::UiElementContainer;
use crate::ui::UiEvent;
use crate::ui::UiEventResponse;
use crate::ui::UiRenderer;

#[derive(Debug, Default)]
pub struct UiSystem {
	root:                  Option<UiElementContainer>,
	event_response_sender: Option<Sender<Box<dyn UiEventResponse>>>,
}

impl UiSystem {
	pub fn setup(
		&mut self,
		_system: &mut System,
		event_response_sender: Sender<Box<dyn UiEventResponse>>,
	) -> anyhow::Result<()> {
		self.event_response_sender = Some(event_response_sender);
		Ok(())
	}

	pub fn teardown(&mut self) {
		if let Some(_ers) = self.event_response_sender.take() {}
		if let Some(_root) = self.root.take() {}
	}

	pub fn set_root(&mut self, root: UiElementContainer) {
		self.root = Some(root);
	}

	pub fn take_root(&mut self) -> Option<UiElementContainer> {
		todo!("Why do you call this? Do we need this");
		#[allow(unreachable_code)]
		self.root.take()
	}

	pub fn set_size(&mut self, size: &Vector2) {
		if let Some(root) = &mut self.root {
			root.set_size(size);
		}
	}

	pub fn layout(&mut self) {
		if let Some(root) = &mut self.root {
			root.layout(&Vector2::zero());
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
						//debug!("{:?}", &root);
						if let Some(ev) = root.handle_ui_event(&ev, &event_response_sender) {
							//println!("Click handled");
							let _ = event_response_sender.send(ev).unwrap();
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
			//			renderer.use_effect( EffectId::ColoredTextured as u16 );

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
			root.render_debug(debug_renderer, &Vector2::zero());
		}
	}
}
