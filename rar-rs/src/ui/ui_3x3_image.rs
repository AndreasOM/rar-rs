use oml_game::math::Matrix32;
use oml_game::math::Vector2;
use tracing::*;

use crate::ui::{UiElement, UiElementContainerData, UiElementFadeState, UiRenderer};

pub struct Ui3x3Image {
	imagename:   String,
	imagesize:   Vector2,
	texturesize: Vector2,
	quads:       Vec<(Vector2, Vector2, Matrix32)>, // offset, size, texture matrix
}

impl Ui3x3Image {
	pub fn new(imagename: &str, size: &Vector2, texturesize: &Vector2) -> Self {
		let div = size.scaled_reciprocal_vector2(&texturesize);
		let _scale = 1.0;
		if div.x.fract() != 0.0 || div.y.fract() != 0.0 {
			dbg!(&div);
			warn!("Ui3x3Image doesn't support non integer repeats!")
		}
		let ts3 = texturesize.scaled(1.0 / 3.0);
		let cx = div.x as isize * 3;
		let cy = div.y as isize * 3;
		let mut quads = Vec::new();
		// Test: one fullsize quad
		//quads.push((Vector2::zero(), *size, Matrix32::identity()));
		//
		let corner = size.sub(&ts3).scaled(0.5);
		let top_left = corner.scaled_vector2(&Vector2::new(-1.0, 1.0));
		// top left
		quads.push((top_left.clone(), ts3.clone(), Matrix32::identity()));

		// top part
		if cy > 0 {
			// should always be true, otherwise we scale
			if cx > 2 {
				for x in 1..cx - 1 {
					quads.push((
						top_left.sub(&ts3.scaled_vector2(&Vector2::new(-x as f32, 0.0))),
						ts3.clone(),
						Matrix32::identity(),
					));
				}
			}
		}
		// top right
		quads.push((
			top_left.sub(&ts3.scaled_vector2(&Vector2::new((1 - cx) as f32, 0.0))),
			ts3.clone(),
			Matrix32::identity(),
		));

		// far left column
		if cx > 0 {
			// should always be true, otherwise we scale
			if cy > 2 {
				for y in 1..cy - 1 {
					quads.push((
						top_left.sub(&ts3.scaled_vector2(&Vector2::new(0.0, y as f32))),
						ts3.clone(),
						Matrix32::identity(),
					));
				}
			}
		}
		// middle part
		if cx > 2 && cy > 2 {
			for y in 1..cy - 1 {
				for x in 1..cx - 1 {
					quads.push((
						top_left.sub(&ts3.scaled_vector2(&Vector2::new(-x as f32, y as f32))),
						ts3.clone(),
						Matrix32::identity(),
					));
				}
			}
		}
		// far right column
		if cx > 2 {
			// should always be true, otherwise we scale
			if cy > 2 {
				for y in 1..cy - 1 {
					quads.push((
						top_left.sub(&ts3.scaled_vector2(&Vector2::new((1 - cx) as f32, y as f32))),
						ts3.clone(),
						Matrix32::identity(),
					));
				}
			}
		}
		// bottom left
		quads.push((
			top_left.sub(&ts3.scaled_vector2(&Vector2::new(0.0, (cy - 1) as f32))),
			ts3.clone(),
			Matrix32::identity(),
		));
		// bottom part
		if cy > 2 {
			// should always be true, otherwise we scale
			if cx > 2 {
				for x in 1..cx - 1 {
					quads.push((
						top_left
							.sub(&ts3.scaled_vector2(&Vector2::new(-x as f32, (cy - 1) as f32))),
						ts3.clone(),
						Matrix32::identity(),
					));
				}
			}
		}
		// bottom right
		quads.push((
			top_left.sub(&ts3.scaled_vector2(&Vector2::new((1 - cx) as f32, (cy - 1) as f32))),
			ts3.clone(),
			Matrix32::identity(),
		));

		Self {
			imagename: imagename.to_owned(),
			imagesize: *size,
			texturesize: *texturesize,
			quads,
		}
	}
}
/*
		let mtx = Matrix32::scaling_xy(1.0 * a, 1.0);
		//mtx.pos.x = - self.pos.x / 1024.0;
		renderer.set_tex_matrix(&mtx);
*/

impl UiElement for Ui3x3Image {
	fn as_any(&self) -> &dyn std::any::Any {
		self
	}
	fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
		self
	}
	fn preferred_size(&self) -> Option<&Vector2> {
		Some(&self.imagesize)
	}
	/*
	fn update( &mut self, _time_step: f64 ) {
	}
	*/
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
				ui_renderer.render_textured_quad(&p, &s);
			}
			ui_renderer.pop_opacity();
		}
	}
}
