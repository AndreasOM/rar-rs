use std::collections::HashMap;

use oml_game::math::Vector2;
use oml_game::renderer::debug_renderer::DebugRenderer;
use oml_game::renderer::Color;
use oml_game::renderer::Renderer;

use crate::rar::effect_ids::EffectId;
use crate::rar::layer_ids::LayerId;
use crate::rar::{map::LayerType, World};

#[derive(Debug, Default)]
pub struct WorldRenderer {
	enabled_layers: HashMap<String, bool>,
}

impl WorldRenderer {
	pub fn setup(&mut self) -> anyhow::Result<()> {
		Ok(())
	}

	pub fn teardown(&mut self) {}

	pub fn enable_layer(&mut self, layer_name: &str) {
		let _layer = self
			.enabled_layers
			.entry(layer_name.to_string())
			.or_insert(true);
	}

	pub fn render(&mut self, renderer: &mut Renderer, world: &World) {
		// :HACK: :TODO: make layers configurable by user
		renderer.use_layer(LayerId::TileMap1 as u8);
		renderer.use_effect(EffectId::Textured as u16);

		//		dbg!(&self);
		for m in world.maps() {
			//			dbg!(&m.filename());
			if let Some(map) = m.map() {
				// dbg!( &map );
				for l in map.layers() {
					if let Some(enabled_layer) = self.enabled_layers.get(l.name()) {
						//						dbg!( &l.name() );
						//						dbg!( &l );
						//						println!("Layer ->");
						match l.layertype() {
							LayerType::Objects => {},
							LayerType::Tile => {
								for c in l.chunks() {
									let tm = c.tile_map();
									let w = tm.width();
									let h = tm.height();

									let ox = *c.x();

									// :TODO: optimise the following four boundaries
									let sy = 0;
									let ey = w;
									let sx = 0;
									let ex = h;
									///									println!("Chunk ->");
									let th = *map.tileheight();
									let tw = *map.tilewidth();
									//									let th = 32;
									//									let tw = 32;
									let size = Vector2::new(tw as f32, th as f32);
									let mut pos = Vector2::new(0.0, 512.0 - 0.5 * (th as f32)); // :TODO: world offset etc

									// :HACK: apply chunk offset
									pos = pos.add( &Vector2::new( ( ox as f32 ) * ( tw as f32 ), 0.0 ) );
									let inc_x = Vector2::new(tw as f32, 0.0);
									// including undo row
									let inc_y =
										Vector2::new(0.0 - ((tw * (ex - sx)) as f32), -(th as f32));
									for y in sy..ey {
										for x in sx..ex {
											let tid = tm.get_xy(x, y);
											if tid == 1 {
												renderer.use_texture("tile_default_block");
												renderer.render_textured_quad(&pos, &size);
											}
											else if tid == 2 {
												renderer.use_texture("tile_default_block_green");
												renderer.render_textured_quad(&pos, &size);
											}
											pos = pos.add(&inc_x);

//											print!("{:1}", tid);
										}
										pos = pos.add(&inc_y);
//										println!();
									}
								}
							},
							LayerType::None => {},
						};
					}
				}
			}
		}
	}
}
