use std::collections::HashMap;

use oml_game::math::Vector2;
use oml_game::renderer::debug_renderer::DebugRenderer;
//use oml_game::renderer::debug_renderer::DebugRenderer;
use oml_game::renderer::Color;
use oml_game::renderer::Renderer;
use tracing::*;

use crate::rar::{
	camera::Camera,
	map,
	map::{Layer, LayerType},
	World,
};

const MAP_TEXT_SCALE: f32 = 20.0;
const MAP_TEXT_WIDTH: f32 = 2.0;

#[derive(Debug, Default)]
struct EnabledLayer {
	layer_id:  u8,
	effect_id: u16,
}

#[derive(Debug, Default)]
pub struct WorldRenderer {
	enabled_layers: HashMap<String, EnabledLayer>,
	total_time:     f64,
}

impl WorldRenderer {
	pub fn setup(&mut self) -> anyhow::Result<()> {
		Ok(())
	}

	pub fn teardown(&mut self) {}

	pub fn enable_layer(&mut self, layer_name: &str, layer_id: u8, effect_id: u16) {
		let mut layer = self
			.enabled_layers
			.entry(layer_name.to_string())
			.or_insert(EnabledLayer::default());

		layer.layer_id = layer_id;
		layer.effect_id = effect_id;
		//		layer.enabled = true;
	}
	pub fn update(&mut self, time_step: f64) {
		self.total_time += time_step;
	}
	pub fn render(&mut self, renderer: &mut Renderer, camera: &Camera, world: &World) {
		//		dbg!(&self);
		let frame = camera.frame();
		let left = frame.left();
		let right = frame.right();
		let _top = frame.top();
		let _bottom = frame.bottom();

		let mut tiles_rendered = 0;
		let mut chunks_with_tiles_rendered = 0;
		for m in world.maps() {
			//			dbg!(&m.filename());
			if let Some(map) = m.map() {
				// dbg!( &map );
				let th = *map.tileheight();
				let tw = *map.tilewidth();

				let tile_left = (left / tw as f32).floor() as i32;
				let tile_right = (right / tw as f32).ceil() as i32;

				for l in map.layers() {
					if let Some(enabled_layer) = self.enabled_layers.get(l.name()) {
						//						dbg!( &l.name() );
						//						dbg!( &l );
						//						println!("Layer ->");
						renderer.use_layer(enabled_layer.layer_id);
						renderer.use_effect(enabled_layer.effect_id);

						match l.layertype() {
							LayerType::Objects => {},
							LayerType::Tile => {
								for c in l.chunks() {
									let tm = c.tile_map();
									let w = tm.width();
									let h = tm.height();
									let ox = *c.x();
									let oy = *c.y();

									// :TODO: optimise the following two boundaries
									let sy = 0;
									let ey = h;
									let sx = (tile_left - ox).clamp(0, w as i32) as u32;
									let ex = (tile_right - ox).clamp(0, w as i32) as u32;

									let size = Vector2::new(tw as f32, th as f32);
									let pos =
										Vector2::new(0.5 * (tw as f32), 512.0 - 0.5 * (th as f32)); // :TODO: world offset etc

									let pos = pos.add(&Vector2::new(
										(sx as f32) * tw as f32,
										-(sy as f32) * th as f32,
									));

									let mut pos = pos.add(&camera.offset());

									// :HACK: apply chunk offset
									pos = pos.add(&Vector2::new(
										(ox as f32) * (tw as f32),
										(oy as f32) * (th as f32),
									));
									let inc_x = Vector2::new(tw as f32, 0.0);
									// including undo row aka carriage return ;)
									let inc_y =
										Vector2::new(0.0 - ((tw * (ex - sx)) as f32), -(th as f32));

									let mut tiles_rendered_in_chunk = 0;
									for y in sy..ey {
										for x in sx..ex {
											tiles_rendered_in_chunk += 1;
											let tid = tm.get_xy(x, y);
											if tid > 0 {
												let image = map.get_tile_image(tid);
												if !image.is_empty() {
													renderer.use_texture(image);
													renderer.render_textured_quad(&pos, &size);
												} else {
													println!("Warning: No tile for TID: {}", tid);
												}
											}
											pos = pos.add(&inc_x);
										}
										pos = pos.add(&inc_y);
									}
									tiles_rendered += tiles_rendered_in_chunk;
									if tiles_rendered_in_chunk > 0 {
										chunks_with_tiles_rendered += 1;
									}
								}
							},
							LayerType::None => {},
						};
					}
				}
			}
		}
		if false {
			println!(
				"Tiles rendered {}, Chunks with tiles {}",
				tiles_rendered, chunks_with_tiles_rendered
			);
		}
	}

	pub fn render_debug_layer_objects(
		&self,
		debug_renderer: &mut DebugRenderer,
		camera: &Camera,
		layer: &Layer,
	) {
		let l = layer;

		let offset = camera.offset();

		//if l.name() == "CameraControl" {
		for o in l.objects() {
			//						dbg!(&o);
			let mut color = Color::rainbow(self.total_time as f32 * 20.0); //Color::white();
			let mut width = 3.0;
			let mut do_debug_render = true;
			match (l.name().as_str(), o.class().as_str()) {
				("Player", "PlayerSpawn") => {
					//color = Color::red();
					width = 9.0;
				},
				("Player", "PlayerKill") => {
					color = Color::red();
					width = 9.0;
				},
				("CameraControl", "CameraStart") => {
					//color = Color::blue();
					width = 9.0;
				},
				("CameraControl", "CameraFreeze") => {
					color = Color::blue();
					width = 9.0;
				},
				("CameraControl", "CameraThaw") => {
					color = Color::green();
					width = 9.0;
				},
				("Collider", _) => {
					color = Color::green();
					width = 3.0;
				},
				(_, "Test") => {
					do_debug_render = false;
				},
				_ => {
					warn!(
						"Object not handled: {}[] {}[{}]",
						l.name(),
						o.name(),
						o.class()
					);
					do_debug_render = false;
				},
			};
			if do_debug_render {
				match o.data() {
					map::ObjectData::Rectangle {
						rect,
						bounding_circle: _,
					} => {
						let mut rect = rect.clone();
						//								let offset = self.camera.scaled_vector2( &Vector2::new( -1.0, 1.0 ) );
						rect.offset(&offset);

						debug_renderer.add_rectangle(&rect, width, &color);
						debug_renderer.add_text(
							&rect.center().add(&Vector2::new(3.0, 3.0)),
							o.class(),
							MAP_TEXT_SCALE,
							MAP_TEXT_WIDTH,
							&Color::black(),
						);
						debug_renderer.add_text(
							&rect.center(),
							o.class(),
							MAP_TEXT_SCALE,
							MAP_TEXT_WIDTH,
							&color,
							//&Color::rainbow(self.total_time as f32 * 20.0),
						);
						//								debug_renderer.add_text(rect.pos(), o.class(), 50.0, 5.0, &Color::from_rgba( 0.75, 0.75, 0.95, 1.0 ));
					},
					map::ObjectData::Point { pos } => {
						let pos = offset.add(pos);
						//let pos = pos.add( &Vector2::new( 0.0, 0.0 ) );
						debug_renderer.add_circle(&pos, 50.0, 5.0, &color);
						debug_renderer.add_text(
							&pos,
							o.class(),
							MAP_TEXT_SCALE,
							MAP_TEXT_WIDTH,
							&color,
						);
					},
					map::ObjectData::Unknown => {},
					/*
					d => {
						println!("Unhandled {:?}", &d);
					},
					*/
				}
			}
		}
		//}
	}
}
