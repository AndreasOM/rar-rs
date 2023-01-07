use oml_game::math::Vector2;
use tracing::*;

use crate::ui::UiElement;
use crate::ui::UiElementContainerData;

#[derive(Debug, Default)]
pub struct UiGridBox {
	padding:      f32,
	column_count: usize,
}

impl UiGridBox {
	pub fn with_padding(mut self, padding: f32) -> Self {
		self.padding = padding;

		self
	}
	pub fn with_column_count(mut self, column_count: usize) -> Self {
		self.column_count = column_count;

		self
	}
}

impl UiElement for UiGridBox {
	fn type_name(&self) -> &str {
		"[UiGridBox]"
	}
	fn as_any(&self) -> &dyn std::any::Any {
		self
	}
	fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
		self
	}
	fn recalculate_size(&mut self, container: &mut UiElementContainerData) {
		let mut total_size = Vector2::zero();

		let padding = self.padding;
		let mut row_width = padding;
		let mut row_height = 0.0;
		let mut x = 0;
		total_size.y += padding;

		// :TODO: padding
		for c in container.borrow_children().iter() {
			let c = c.borrow();
			let cs = c.size();
			row_width += cs.x + padding;
			let padded_height = cs.y + padding;
			if padded_height > row_height {
				row_height = padded_height;
			}
			x += 1;
			if x >= self.column_count {
				debug!("{} {} {}", x, row_width, row_height);
				if row_width > total_size.x {
					total_size.x = row_width;
				}
				total_size.y += row_height;
				x = 0;
				row_width = padding;
				row_height = padding;
			}
		}
		if x > 0 {
			debug!("{} {} {}", x, row_width, row_height);
			if row_width > total_size.x {
				total_size.x = row_width;
			}
			total_size.y += row_height;
		}

		debug!("total_size {:?}", &total_size);
		//		total_size.x -= self.padding;

		container.set_size(&total_size);
	}

	fn layout(&mut self, container: &mut UiElementContainerData, pos: &Vector2) {
		let padding = self.padding;

		let mut column_starts = Vec::new();
		column_starts.resize(self.column_count + 1, 0.0);
		let mut row_starts: Vec<f32> = Vec::new();

		let mut row_width = 0.5 * padding;
		let mut row_height = 0.0;

		let mut total_height = -0.5 * padding;
		row_starts.push(total_height);
		let mut x = 0;
		let mut y = 0;

		// :TODO: .chunks( self.column_count) might be simpler
		for c in container.borrow_children().iter() {
			let c = c.borrow();
			let cs = c.size();
			row_width += cs.x + padding; // technically 2*0.5*padding ;)
			let padded_height = cs.y + padding; // technically 2*0.5*padding ;)
			if padded_height > row_height {
				row_height = padded_height;
			}
			x += 1;
			if column_starts[x] < row_width {
				column_starts[x] = row_width;
			}

			if x >= self.column_count {
				y += 1;
				total_height -= row_height;
				row_starts.push(total_height);
				x = 0;
				row_width = 0.5 * padding;
				row_height = 0.0;
			}
		}

		let row_centers: Vec<f32> = row_starts
			.windows(2)
			.map(|w| 0.5 * w[0] + 0.5 * w[1])
			.collect();
		let column_centers: Vec<f32> = column_starts
			.windows(2)
			.map(|w| 0.5 * w[0] + 0.5 * w[1])
			.collect();
		debug!("row_starts {:#?}", &row_starts);
		debug!("column_starts {:#?}", &column_starts);
		debug!("row_centers {:#?}", &row_centers);
		debug!("column_centers {:#?}", &column_centers);
		let mut x = 0;
		let mut y = 0;
		let mut cpos = Vector2::zero();
		debug!("{:?}", &container.size);
		let h = container.size.scaled_vector2(&Vector2::new(-0.5, 0.5));
		for c in container.borrow_children_mut().iter_mut() {
			cpos.x = column_centers[x];
			cpos.y = row_centers[y];
			debug!("cpos = {:?}", &cpos);
			//			let hsize = c.borrow().size().scaled_vector2( &h );
			//			let p = cpos.sub( &hsize );
			let p = cpos.add(&h);
			c.borrow_mut().layout(&p);
			x += 1;

			if x >= self.column_count {
				x = 0;
				y += 1;
			}
		}
		container.set_pos(pos);
		//container.set_pos( &Vector2::new( 180.0, 0.0 ) );
		//todo!();
		/*
		//debug!("{}", container.name());
		let mut total_size = Vector2::zero();
		let mut c_positions_x = Vec::new();
		let padding = self.padding;

		let mut w1 = 0.0;

		for c in container.borrow_children().iter() {
			let c = c.borrow();
			let cs = c.size();
			total_size.x += cs.x + padding;
			if total_size.y < cs.y {
				total_size.y = cs.y;
			}
			let w0 = w1;
			w1 = 0.5 * cs.x;
			c_positions_x.push(w0 + w1);
		}
		total_size.x -= padding;

		c_positions_x.push(0.0);

		let mut cpos = Vector2::new(-0.5 * total_size.x - self.padding, 0.0);

		for (i, c) in container.borrow_children_mut().iter_mut().enumerate() {
			let x = c_positions_x[i];
			cpos.x += x + padding;
			c.borrow_mut().layout(&cpos);
		}

		container.set_pos(pos);
		//container.set_size(&total_size);
		*/
	}
}
