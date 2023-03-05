use oml_game::math::Matrix44;
use oml_game::renderer::Renderer;
use oml_game::system::System;
use oml_game::window::WindowUpdateContext;
use oml_game_egui::EguiWrapper;

use crate::rar::effect_ids::EffectId;
use crate::rar::layer_ids::LayerId;

#[derive(Debug, Default)]
pub struct RarAppEgui {
	wrapper:              EguiWrapper,
	enabled:              bool,
	debug_telemetry_open: bool,
	telemetry:            oml_game_egui::EguiTelemetryWidget,
	is_done:              bool,
}

impl RarAppEgui {
	pub fn setup(&mut self, scale_factor: f32) {
		self.wrapper.setup(scale_factor);
		self.wrapper
			.set_effect_id(EffectId::ColoredTexturedEgui as u16);
		self.wrapper.set_layer_id(LayerId::Egui as u8);
	}

	fn debug_telemetry_window(&mut self, ctx: &egui::Context) {
		egui::Window::new("telemetry")
			.open(&mut self.debug_telemetry_open)
			.default_size(egui::vec2(400.0, 400.0))
			.vscroll(false)
			.show(ctx, |ui| {
				ui.label("Telemetry");
			});
	}

	pub fn update(&mut self, system: &mut System, wuc: &mut WindowUpdateContext) {
		if wuc.was_key_pressed('`' as u8) {
			self.enabled = !self.enabled;
		}

		if self.enabled {
			self.wrapper.update(wuc);

			self.wrapper.run(system, |ctx| {
				if true {
					let mut style = (*ctx.style()).clone();
					style.override_text_style = Some(
						egui::TextStyle::Heading, //egui::FontId::new(30.0, egui::FontFamily::Proportional)
					);

					let font_scale = 0.5;
					style.text_styles = [
						(
							egui::TextStyle::Heading,
							egui::FontId::new(font_scale * 30.0, egui::FontFamily::Proportional),
						),
						(
							egui::TextStyle::Name("Heading2".into()),
							egui::FontId::new(font_scale * 25.0, egui::FontFamily::Proportional),
						),
						(
							egui::TextStyle::Name("Context".into()),
							egui::FontId::new(font_scale * 23.0, egui::FontFamily::Proportional),
						),
						(
							egui::TextStyle::Body,
							egui::FontId::new(font_scale * 18.0, egui::FontFamily::Proportional),
						),
						(
							egui::TextStyle::Monospace,
							egui::FontId::new(font_scale * 14.0, egui::FontFamily::Proportional),
						),
						(
							egui::TextStyle::Button,
							egui::FontId::new(font_scale * 14.0, egui::FontFamily::Proportional),
						),
						(
							egui::TextStyle::Small,
							egui::FontId::new(font_scale * 10.0, egui::FontFamily::Proportional),
							//egui::FontId::new(self.font_size as f32, egui::FontFamily::Proportional),
						),
					]
					.into();

					ctx.set_style(style);
				}
				//				self.egui_debug_telemetry_window( ctx );

				egui::Window::new("telemetry")
					.open(&mut self.debug_telemetry_open)
					.default_size(egui::vec2(400.0, 400.0))
					.vscroll(false)
					.show(ctx, |ui| {
						ui.label("Telemetry");
						self.telemetry.show(ui);
					});

				//self.egui_debug_telemetry_window( ctx );

				egui::SidePanel::right("egui_debug")
					.resizable(false)
					.default_width(150.0)
					.show(ctx, |ui| {
						egui::trace!(ui);
						ui.vertical_centered(|ui| {
							ui.heading("-= Debug =-");
						});
						if ui.button("Quit").clicked() {
							self.is_done = true;
							// frame.quit();
						}

						ui.separator();
						egui::ScrollArea::vertical().show(ui, |ui| {
							ui.with_layout(
								egui::Layout::top_down_justified(egui::Align::LEFT),
								|ui| {
									ui.toggle_value(&mut self.debug_telemetry_open, "Telemetry");
								},
							);
						});
					});
				Ok(())
			});
		}
	}
	pub fn render(&mut self, system: &mut System, renderer: &mut Renderer) {
		if self.enabled {
			self.wrapper.render(system, renderer);

			let viewport_size = renderer.viewport_size();
			let scaling = 0.5;
			let left = -viewport_size.x * scaling;
			let right = viewport_size.x * scaling;
			let top = viewport_size.y * scaling;
			let bottom = -viewport_size.y * scaling;
			let near = 1.0;
			let far = -1.0;

			let mvp = Matrix44::ortho(left, right, bottom, top, near, far);

			renderer.set_uniform_matrix44("modelViewProjectionMatrixReal", mvp);
		}
	}
}
