use std::collections::BTreeMap;

use oml_game::math::Matrix44;
use oml_game::renderer::Color;
use oml_game::renderer::Renderer;
use oml_game::system::System;
use oml_game::window::WindowUpdateContext;
use oml_game_egui::EguiWrapper;

use crate::rar::effect_ids::EffectId;
use crate::rar::layer_ids::LayerId;

#[derive(Debug, Default)]
pub struct RarAppEgui {
	wrapper:      EguiWrapper,
	is_done:      bool,
	state:        State,
	active_color: Color,
	ghost_color:  Color,
	windows:      BTreeMap<String, Window>,
}
#[derive(Debug)]
pub struct Window {
	open:   bool,
	window: Box<dyn EguiDebugWindow>,
}

#[derive(Debug, Default, PartialEq)]
enum State {
	#[default]
	Disabled,
	Enabled,
	Ghost, // visible, but not interactable
}

impl State {
	pub fn cycle(&self) -> Self {
		use State::*;
		match *self {
			Disabled => Enabled,
			Enabled => Ghost,
			Ghost => Disabled,
		}
	}
	pub fn needs_render(&self) -> bool {
		use State::*;
		match *self {
			Disabled => false,
			_ => true,
		}
	}
	pub fn input_disabled(&self) -> bool {
		use State::*;
		match *self {
			Enabled => false,
			_ => true,
		}
	}
}

impl RarAppEgui {
	pub fn setup(&mut self, scale_factor: f32) {
		self.wrapper.setup(scale_factor);
		self.wrapper
			.set_effect_id(EffectId::ColoredTexturedEgui as u16);
		self.wrapper.set_layer_id(LayerId::Egui as u8);

		self.ghost_color = Color::from_rgba(1.0, 0.8, 0.8, 0.8);
		let t = TelemetryWindow::default();
		self.register_window(Box::new(t));
	}

	pub fn register_window(&mut self, window: Box<dyn EguiDebugWindow>) {
		let name = window.name();
		let window = Window {
			open:   false,
			window: window,
		};
		self.windows.insert(name.to_string(), window);
	}

	pub fn find_window_as_and_then<W>(&mut self, name: &str, mut f: impl FnMut(&mut W))
	where
		W: EguiDebugWindow + 'static,
	{
		if let Some(w) = self.windows.get_mut(name) {
			let w = &mut w.window;
			match w.as_any_mut().downcast_mut::<W>() {
				Some(w) => {
					f(w);
				},
				None => panic!(
					// :TODO: maybe this is ok!?
					"{:?} isn't a {:?} with name {:#?}!",
					&w,
					std::any::type_name::<W>(),
					&name,
				),
			}
		}
	}

	// :TODO: deregister?

	pub fn update(&mut self, system: &mut System, wuc: &mut WindowUpdateContext) {
		if wuc.was_key_pressed('`' as u8) {
			self.state = self.state.cycle();
		}

		if self.state.needs_render() {
			self.wrapper.set_input_disabled(self.state.input_disabled());
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

				/*
				egui::Window::new("telemetry")
					.open(&mut self.debug_telemetry_open)
					.default_size(egui::vec2(400.0, 400.0))
					.vscroll(false)
					.show(ctx, |ui| {
						ui.label("Telemetry");
						self.telemetry.show(ui);
					});

				*/
				//self.egui_debug_telemetry_window( ctx );

				for (name, w) in self.windows.iter_mut() {
					w.window.display(ctx, &mut w.open);
				}

				egui::SidePanel::right("egui_debug")
					.resizable(false)
					.default_width(150.0)
					.show(ctx, |ui| {
						egui::trace!(ui);
						ui.vertical_centered(|ui| {
							ui.heading("-= Debug =-");
							ui.label(format!("{:?}", self.state));
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
									//ui.toggle_value(&mut self.debug_telemetry_open, "Telemetry");
									for (name, w) in self.windows.iter_mut() {
										//										ui.toggle_value(&mut self.debug_telemetry_open, name);
										ui.toggle_value(&mut w.open, name);
									}
								},
							);
						});
					});
				Ok(())
			});
		}
	}
	pub fn render(&mut self, system: &mut System, renderer: &mut Renderer) {
		if self.state.needs_render() {
			let color = if self.state == State::Ghost {
				&self.ghost_color
			} else {
				&self.active_color
			};

			self.wrapper.set_color(color);
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

#[derive(Default, Debug)]
struct TelemetryWindow {
	telemetry: oml_game_egui::EguiTelemetryWidget,
}

impl EguiDebugWindow for TelemetryWindow {
	fn as_any(&self) -> &dyn std::any::Any {
		self
	}
	fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
		self
	}
	fn name(&self) -> &'static str {
		"Telemetry"
	}

	fn display(&mut self, ctx: &egui::Context, open: &mut bool) {
		egui::Window::new(self.name())
			.open(open)
			.default_size(egui::vec2(400.0, 400.0))
			//.vscroll(false)
			.show(ctx, |ui| {
				self.telemetry.show(ui);
			});
	}
}

pub trait EguiDebugWindow: std::fmt::Debug {
	fn as_any(&self) -> &dyn std::any::Any;
	fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
	fn name(&self) -> &'static str;
	fn display(&mut self, ctx: &egui::Context, open: &mut bool);
}
