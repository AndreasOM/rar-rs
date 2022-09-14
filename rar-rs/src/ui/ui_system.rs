use oml_game::renderer::Renderer;
use oml_game::system::System;

use crate::rar::effect_ids::EffectId;
use crate::rar::layer_ids::LayerId;
use crate::rar::AppUpdateContext;
use crate::ui::UiElementContainer;
use crate::ui::UiRenderer;

#[derive(Debug, Default)]
pub struct UiSystem {
	root: Option<UiElementContainer>,
}

impl UiSystem {
	pub fn setup(&mut self, _system: &mut System) -> anyhow::Result<()> {
		Ok(())
	}

	pub fn teardown(&mut self) {
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

	pub fn update(&mut self, auc: &mut AppUpdateContext) {
		if let Some(root) = &mut self.root {
			if let Some(wuc) = auc.wuc() {
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
}
