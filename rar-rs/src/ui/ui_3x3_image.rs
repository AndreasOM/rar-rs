use oml_game::math::Matrix32;
use oml_game::math::Vector2;
use serde::Deserialize;
use tracing::*;

use crate::ui::UiElementInfo;
use crate::ui::{UiElement, UiElementContainerData, UiElementFadeState, UiRenderer};

#[derive(Debug, Default)]
pub struct Ui3x3Image {
	imagename:   String,
	imagesize:   Vector2,
	texturesize: Vector2,
	quads:       Vec<(Vector2, Vector2, Matrix32)>, // offset, size, texture matrix
}

impl Ui3x3Image {
	pub fn new(imagename: &str, size: &Vector2, texturesize: &Vector2) -> Self {
		let mut s = Self {
			imagename:   imagename.to_owned(),
			imagesize:   *size,
			texturesize: *texturesize,
			quads:       Vec::new(),
		};

		s.recalculate_quads();
		s
	}
	fn recalculate_quads(&mut self) {
		let texturesize = &self.texturesize;
		let size = &self.imagesize;
		// Note: the whole quad handling is a massive :HACK: there are much better ways to create, and render the mesh
		let mut div = size.scaled_reciprocal_vector2(&texturesize);
		let mut scale = Vector2::new(1.0 / 3.0, 1.0 / 3.0);
		if div.x.fract() != 0.0 {
			info!("Ui3x3Image with non integer X repeat {}!", div.x);
			let fx = div.x.floor();
			scale.x *= div.x / fx;
			div.x = fx;
		}
		if div.y.fract() != 0.0 {
			info!("Ui3x3Image with non integer Y repeat {}!", div.y);
			let fy = div.y.floor();
			scale.y *= div.y / fy;
			div.y = fy;
		}
		let ts3 = texturesize.scaled_vector2(&scale);
		let cx = div.x as isize * 3;
		let cy = div.y as isize * 3;
		let mut quads = Vec::new();

		let corner = size.sub(&ts3).scaled(0.5);
		let top_left = corner.scaled_vector2(&Vector2::new(-1.0, 1.0));
		// top left
		let mtx13 = Matrix32::identity().with_scaling(1.0 / 3.0);

		quads.push((top_left.clone(), ts3.clone(), mtx13.clone()));

		// top part
		let mtx = mtx13
			.clone()
			.with_translation(&Vector2::new(1.0 / 3.0, 0.0));
		if cy > 0 {
			// should always be true, otherwise we scale
			if cx > 2 {
				for x in 1..cx - 1 {
					quads.push((
						top_left.sub(&ts3.scaled_vector2(&Vector2::new(-x as f32, 0.0))),
						ts3.clone(),
						mtx.clone(),
					));
				}
			}
		}
		// top right
		let mtx = mtx13
			.clone()
			.with_translation(&Vector2::new(2.0 / 3.0, 0.0));
		quads.push((
			top_left.sub(&ts3.scaled_vector2(&Vector2::new((1 - cx) as f32, 0.0))),
			ts3.clone(),
			mtx.clone(),
		));

		// far left column
		let mtx = mtx13
			.clone()
			.with_translation(&Vector2::new(0.0 / 3.0, 1.0 / 3.0));
		if cx > 0 {
			// should always be true, otherwise we scale
			if cy > 2 {
				for y in 1..cy - 1 {
					quads.push((
						top_left.sub(&ts3.scaled_vector2(&Vector2::new(0.0, y as f32))),
						ts3.clone(),
						mtx.clone(),
					));
				}
			}
		}
		// middle part
		let mtx = mtx13
			.clone()
			.with_translation(&Vector2::new(1.0 / 3.0, 1.0 / 3.0));
		if cx > 2 && cy > 2 {
			for y in 1..cy - 1 {
				for x in 1..cx - 1 {
					quads.push((
						top_left.sub(&ts3.scaled_vector2(&Vector2::new(-x as f32, y as f32))),
						ts3.clone(),
						mtx.clone(),
					));
				}
			}
		}
		// far right column
		let mtx = mtx13
			.clone()
			.with_translation(&Vector2::new(2.0 / 3.0, 1.0 / 3.0));
		if cx > 2 {
			// should always be true, otherwise we scale
			if cy > 2 {
				for y in 1..cy - 1 {
					quads.push((
						top_left.sub(&ts3.scaled_vector2(&Vector2::new((1 - cx) as f32, y as f32))),
						ts3.clone(),
						mtx.clone(),
					));
				}
			}
		}
		// bottom left
		let mtx = mtx13
			.clone()
			.with_translation(&Vector2::new(0.0 / 3.0, 2.0 / 3.0));
		quads.push((
			top_left.sub(&ts3.scaled_vector2(&Vector2::new(0.0, (cy - 1) as f32))),
			ts3.clone(),
			mtx.clone(),
		));
		// bottom part
		let mtx = mtx13
			.clone()
			.with_translation(&Vector2::new(1.0 / 3.0, 2.0 / 3.0));
		if cy > 2 {
			// should always be true, otherwise we scale
			if cx > 2 {
				for x in 1..cx - 1 {
					quads.push((
						top_left
							.sub(&ts3.scaled_vector2(&Vector2::new(-x as f32, (cy - 1) as f32))),
						ts3.clone(),
						mtx.clone(),
					));
				}
			}
		}
		// bottom right
		let mtx = mtx13
			.clone()
			.with_translation(&Vector2::new(2.0 / 3.0, 2.0 / 3.0));
		quads.push((
			top_left.sub(&ts3.scaled_vector2(&Vector2::new((1 - cx) as f32, (cy - 1) as f32))),
			ts3.clone(),
			mtx.clone(),
		));

		self.quads = quads;
	}
	pub fn info() -> &'static UiElementInfo {
		&UiElementInfo {
			type_name:   "Ui3x3Image",
			producer_fn: &Self::produce,
		}
	}

	pub fn produce() -> Box<dyn UiElement> {
		Box::new(Self::default())
	}
}

impl UiElement for Ui3x3Image {
	fn type_name(&self) -> &str {
		"[Ui3x3Image]"
	}
	fn as_any(&self) -> &dyn std::any::Any {
		self
	}
	fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
		self
	}
	fn preferred_size(&self) -> Option<&Vector2> {
		Some(&self.imagesize)
	}
	fn render(&self, container: &UiElementContainerData, ui_renderer: &mut UiRenderer) {
		if *container.fade_state() != UiElementFadeState::FadedOut {
			let l = container.get_fade_level();
			ui_renderer.push_opacity(l);
			ui_renderer.use_texture(&self.imagename);
			// render 3x3 sub images instead of one big one
			// ui_renderer.render_textured_quad(&container.pos, &self.imagesize);
			// Note: actually even more, since the renderer doesn't support texture repeating ... yet(?)
			// Note: We pre-calc everything in the ::new()

			for q in &self.quads {
				let p = &q.0; // :TODO: I am pretty sure we need to factor in the container.pos here ... somehow ... maybe
				let s = &q.1;
				let m = &q.2;
				ui_renderer.push_texture_matrix(&m);
				ui_renderer.render_textured_quad(&p, &s);
				ui_renderer.pop_texture_matrix();
			}
			ui_renderer.pop_opacity();
		}
	}
	fn configure_from_yaml_value(&mut self, yaml_value: serde_yaml::Value) {
		let config: Ui3x3ImageConfig = serde_yaml::from_value(yaml_value).unwrap();

		self.imagesize = Vector2::from_x_str(&config.size);
		self.imagename = config.image;
		self.texturesize = Vector2::from_x_str(&config.texturesize);
		self.recalculate_quads();
	}
}

#[derive(Debug, Deserialize)]
struct Ui3x3ImageConfig {
	image:       String,
	size:        String,
	texturesize: String,
}
