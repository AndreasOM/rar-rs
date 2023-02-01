use std::collections::HashSet;

use derive_getters::Getters;
use oml_game::math::{Circle, Rectangle, Vector2};
use oml_game::system::System;
use tracing::*;

use crate::rar::Tileset;

/* we could use an enum for the different layer types, but for now we just mix into on struct?!
#[derive(Debug)]
enum Layer {
	Objects
}
*/
#[derive(Debug, Default)]
pub enum LayerType {
	Objects,
	Tile,
	#[default]
	None,
}

#[derive(Debug, Default)]
pub enum ObjectData {
	Rectangle {
		rect:            Rectangle,
		bounding_circle: Option<Circle>,
	},
	Point {
		pos: Vector2,
	},
	#[default]
	Unknown,
}

#[derive(Debug, Default, Getters)]
pub struct Object {
	name:  String,
	class: String,
	data:  ObjectData,
}

impl Object {
	pub fn with_data(mut self, data: ObjectData) -> Self {
		self.data = data;

		self
	}
	pub fn hflip(&mut self, pivot_y: f32) {
		let data: &mut ObjectData = &mut self.data;
		//		let mut u = ObjectData::Unknown;
		//		data = &mut u;
		match data {
			ObjectData::Rectangle {
				rect,
				bounding_circle: _,
			} => {
				rect.hflip(pivot_y);
			},
			ObjectData::Point { pos } => {
				pos.y = pivot_y - pos.y;
			},
			_ => {
				panic!("Warning: hflip for {:?} not implemented", &data);
			},
		}
	}
}

//#[derive(Debug)]
#[derive(Clone)]
pub struct TileMap {
	width:  u32,
	height: u32,
	tiles:  Vec<u32>,
}

impl TileMap {
	pub fn new(width: u32, height: u32) -> Self {
		Self {
			width,
			height,
			tiles: Vec::with_capacity((width * height) as usize),
		}
	}

	pub fn push(&mut self, tile: u32) {
		self.tiles.push(tile);
	}

	pub fn width(&self) -> u32 {
		self.width
	}
	pub fn height(&self) -> u32 {
		self.height
	}
	pub fn get_xy(&self, x: u32, y: u32) -> u32 {
		let o = (y * self.width + x) as usize;
		if o < self.tiles.len() {
			self.tiles[o]
		} else {
			0 // tile ZERO is special
		}
	}
}

impl Default for TileMap {
	fn default() -> Self {
		Self {
			width:  0,
			height: 0,
			tiles:  Vec::default(),
		}
	}
}

impl std::fmt::Debug for TileMap {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		//write!(f, "TileMap {}x{} ({} / {})", self.width, self.height, self.)
		let mut f = f.debug_struct("TileMap");
		f.field("width", &self.width)
			.field("width", &self.width)
			.field("height", &self.height)
			.field("cap()", &self.tiles.capacity())
			.field("len()", &self.tiles.len());
		//		f.write_fmt(f, "Hi: {}", 3);
		// :TODO: I wish there was a way to append to a result :(
		f.finish()
	}
}

#[derive(Debug, Default, Getters)]
pub struct Chunk {
	x:        i32,
	y:        i32,
	height:   u32,
	width:    u32,
	tile_map: TileMap,
}

#[derive(Debug, Default, Getters)]
pub struct MapTileset {
	firstgid: u32,
	source:   String,
	tileset:  Option<Tileset>,
}

#[derive(Debug, Default, Getters)]
pub struct Layer {
	layertype: LayerType,
	name:      String,
	objects:   Vec<Object>,
	chunks:    Vec<Chunk>,
}

impl Layer {
	pub fn set_name(&mut self, name: &str) {
		self.name = name.to_string();
	}
	pub fn list_objects_for_class(&self, class: &str) -> Vec<&Object> {
		let mut r = Vec::new();

		for o in self.objects.iter() {
			if o.class() == class {
				r.push(o);
			}
		}
		r
	}

	pub fn list_objects(&self) -> Vec<&Object> {
		let mut r = Vec::new();

		for o in self.objects.iter() {
			r.push(o);
		}
		r
	}

	pub fn add_chunk(&mut self, chunk: Chunk) {
		self.chunks.push(chunk);
	}
	pub fn add_object(&mut self, object: Object) {
		self.objects.push(object);
	}
	pub fn hflip(&mut self, pivot_y: f32) {
		for o in &mut self.objects {
			o.hflip(pivot_y);
			/*
				*/
		}
	}
}

#[derive(Debug, Default, Getters)]
pub struct Map {
	layers:     Vec<Layer>,
	tilesets:   Vec<MapTileset>,
	upsideup:   bool,
	tileheight: u32,
	tilewidth:  u32,
}

impl Map {
	pub fn new() -> Self {
		Self {
			upsideup: true,
			..Default::default()
		}
	}
	pub fn list_objects_in_layer_for_class(&self, layer: &str, class: &str) -> Vec<&Object> {
		let mut r = Vec::new();

		for l in self.layers.iter() {
			if l.name() == layer {
				let mut rl = l.list_objects_for_class(class);
				r.append(&mut rl);
			}
		}
		r
	}

	pub fn list_objects_in_layer(&self, layer: &str) -> Vec<&Object> {
		let mut r = Vec::new();

		for l in self.layers.iter() {
			if l.name() == layer {
				let mut rl = l.list_objects();
				r.append(&mut rl);
			}
		}
		r
	}

	pub fn add_layer(&mut self, layer: Layer) {
		self.layers.push(layer);
	}

	pub fn add_tileset(&mut self, tileset: MapTileset) {
		self.tilesets.push(tileset);
	}

	pub fn load_all_tilesets(&mut self, system: &mut System) -> anyhow::Result<()> {
		for ts in self.tilesets.iter_mut() {
			let mut tileset = Tileset::new();
			tileset.enable_remove_path();
			tileset.load(system, &ts.source)?;

			ts.tileset = Some(tileset);
		}

		Ok(())
	}

	pub fn generate_collider_layers(
		&mut self,
		name: &str,
		layers: &Vec<&str>,
	) -> anyhow::Result<()> {
		if let Some(layer) = self.layers.iter().find(|l| l.name == name) {
			warn!("Layer >{}< already exists -> {:#?}", &name, layer);
			anyhow::bail!("Layer already exists: {}", &name);
		}

		let mut layer = Layer::default();
		layer.set_name(name);

		let mut all_tiles: HashSet<(i32, i32)> = HashSet::new();
		// for now we just use data from *all* layers
		let tile_size = Vector2::new(self.tilewidth as f32, self.tileheight as f32);
		let half_tile_size = tile_size.scaled(0.5);
		let mut min_x = i32::MAX;
		let mut min_y = i32::MAX;
		let mut max_x = i32::MIN;
		let mut max_y = i32::MIN;

		for l in self.layers.iter() {
			if !layers.iter().any(|ul| l.name().starts_with(ul)) {
				debug!(
					"Skipping layer >{}< as for Collider layer >{}<",
					l.name(),
					name
				);
				continue;
			} else {
				// debug!("Using layer >{}< as for Collider layer >{}< -> {:#?}", l.name(), name, &l );
			}
			/* ignore objects for now
			for o in l.objects().iter() {

			}
			*/
			for c in l.chunks().iter() {
				let chunk_x = *c.x();
				let chunk_y = *c.y();
				let chunk_y = chunk_y - 7; // :HACK: what the fudge? 7?
				let tm = c.tile_map();
				// no visibility checks here, it's pre-processed anyway
				for y in 0..*c.height() {
					for x in 0..*c.width() {
						// :TODO: maybe we need the tilemap here to lookup non-1x1 tiles?
						let tid = tm.get_xy(x, y);
						if tid > 0 {
							let cx = chunk_x + x as i32;
							let cy = chunk_y + y as i32;
							all_tiles.insert((cx, cy));
							if cx < min_x {
								min_x = cx;
							} else if cx > max_x {
								max_x = cx;
							}
							if cy < min_y {
								min_y = cy;
							} else if cy > max_y {
								max_y = cy;
							}
						}
					}
				}
			}
		}

		// :HACK: just create one collider per none-zero tile
		'outer: for cy in min_y..=max_y {
			for cx in min_x..=max_x {
				if all_tiles.is_empty() {
					break 'outer;
				}
				if all_tiles.contains(&(cx, cy)) {
					let (x, y) = all_tiles.take(&(cx, cy)).unwrap();
					let start = Vector2::zero(); //tile_size.scaled_vector2(&Vector2::new(x as f32, y as f32));
					let pos = start
						.add(&tile_size.scaled_vector2(&Vector2::new(x as f32, y as f32 * -1.0)));
					let mut pos = pos.add(&half_tile_size);
					let mut rect_size = tile_size;
					let t10 = (x + 1, y + 0);
					let t01 = (x + 0, y + 1);
					let t11 = (x + 1, y + 1);
					match (
						all_tiles.contains(&t10),
						all_tiles.contains(&t01),
						all_tiles.contains(&t11),
					) {
						(true, true, true) => {
							all_tiles.remove(&t10);
							all_tiles.remove(&t01);
							all_tiles.remove(&t11);
							rect_size.x *= 2.0;
							rect_size.y *= 2.0;
							pos.x += half_tile_size.x;
							pos.y -= half_tile_size.y;
						},
						(true, _, _) => {
							all_tiles.remove(&t10);
							rect_size.x *= 2.0;
							pos.x += half_tile_size.x;
						},
						(_, true, _) => {
							all_tiles.remove(&t01);
							rect_size.y *= 2.0;
							pos.y -= half_tile_size.y;
						},
						_ => {},
					};

					let rect = Rectangle::default().with_size(&rect_size).with_center(&pos);
					let bounding_circle = rect.calculate_bounding_circle();
					let od = ObjectData::Rectangle {
						rect,
						bounding_circle: Some(bounding_circle),
					};
					let o = Object::default().with_data(od);
					layer.add_object(o);
				}
			}
		}

		self.add_layer(layer);

		Ok(())
	}

	pub fn load(&mut self, system: &mut System, name: &str) -> anyhow::Result<()> {
		//		return anyhow::bail!("Just testing...");

		let tmj_name = format!("{}.tmj", &name);
		dbg!(&tmj_name);
		if system.default_filesystem().exists(&tmj_name) {
			println!("Trying to load map from {}", &tmj_name);
			let mut map_tmj = MapTmj::new();
			map_tmj.load(system, &tmj_name)?;

			dbg!(&map_tmj);

			*self = map_tmj.into();
		} else {
			anyhow::bail!("No remaining loader for map: {}", &name);
		}
		Ok(())
	}

	pub fn hflip(&mut self, pivot_y: f32) {
		for l in &mut self.layers {
			l.hflip(pivot_y);
		}
		self.upsideup = !self.upsideup;
	}

	pub fn get_tile_image(&self, tid: u32) -> &str {
		// :TODO: combine tileset on load?!
		for mts in self.tilesets.iter() {
			if let Some(ts) = &mts.tileset {
				if tid >= mts.firstgid {
					let image = ts.get_tile_image(tid - mts.firstgid);
					if !image.is_empty() {
						return image;
					}
				}
			}
		}
		//"tile_default_block"
		""
	}
}

impl From<&map_tmj::Chunk> for Chunk {
	fn from(ctmj: &map_tmj::Chunk) -> Self {
		Self {
			x:        *ctmj.x(),
			y:        *ctmj.y(),
			height:   *ctmj.height(),
			width:    *ctmj.width(),
			tile_map: ctmj.tiles().clone(),
		}
	}
}

impl From<&map_tmj::Object> for Object {
	fn from(otmj: &map_tmj::Object) -> Self {
		let data = if otmj.point() {
			//			ObjectData::Unknown
			ObjectData::Point {
				pos: (otmj.x(), otmj.y()).into(),
			}
		} else {
			ObjectData::Rectangle {
				rect:            (otmj.x(), otmj.y(), otmj.width(), otmj.height()).into(),
				bounding_circle: None,
			}
		};

		Self {
			name: otmj.name().to_owned(),
			class: otmj.class().to_owned(),
			data,
		}
	}
}
impl From<&map_tmj::Tileset> for MapTileset {
	fn from(tstmj: &map_tmj::Tileset) -> Self {
		let source = tstmj.source();
		let source = source
			.split("/")
			.last()
			.unwrap_or(source)
			.split(".")
			.nth(0)
			.unwrap_or(source)
			.to_owned();
		Self {
			firstgid: *tstmj.firstgid(),
			source,
			..Default::default()
		}
	}
}

impl From<&map_tmj::Layer> for Layer {
	fn from(ltmj: &map_tmj::Layer) -> Self {
		let mut l = Self {
			name: ltmj.name().to_owned(),
			..Default::default()
		};
		l.layertype = match ltmj.layertype() {
			"objectgroup" => {
				if let Some(objects) = &ltmj.objects() {
					for o in objects {
						let obj = o.into();
						l.add_object(obj);
					}
				}
				LayerType::Objects
			},
			"tilelayer" => {
				if let Some(chunks) = &ltmj.chunks() {
					for c in chunks {
						let chunk = c.into();
						l.add_chunk(chunk);
					}
				}
				LayerType::Tile
			},
			_ => LayerType::None,
		};

		l
	}
}

impl From<map_tmj::MapTmj> for Map {
	fn from(mtmj: map_tmj::MapTmj) -> Self {
		let mut m = Map::new();
		m.tilewidth = mtmj.tilewidth();
		m.tileheight = mtmj.tileheight();
		m.upsideup = false; // tiled is upside down!
		for l in mtmj.layers() {
			m.add_layer(l.into());
		}

		for ts in mtmj.tilesets() {
			m.add_tileset(ts.into());
		}

		m
	}
}

#[path = "./map_tmj.rs"]
mod map_tmj;
use map_tmj::MapTmj;

#[cfg(test)]
mod tests {
	use oml_game::system::filesystem_disk::FilesystemDisk;
	use oml_game::system::System;

	// use crate::rar::Map;
	use super::*;

	#[test]
	fn map_loading_works() -> anyhow::Result<()> {
		let run_test = || -> anyhow::Result<()> {
			let cwd = std::env::current_dir().unwrap();
			let cwd = cwd.to_string_lossy();

			let datadir = format!("{}/../rar-data", &cwd);

			let mut dfs = FilesystemDisk::new(&datadir);

			let mut system = System::new();

			system.set_default_filesystem(Box::new(dfs));

			let mut map = Map::new();
			map.load(&mut system, "world-dev")?;
			//			return anyhow::bail!("Just testing...");

			dbg!(&map);
			Ok(())
		};

		match run_test() {
			Ok(_) => {},
			Err(e) => {
				panic!("Error: {:?}", &e);
			},
		};

		let result = 2 + 2;
		assert_eq!(result, 4);

		Ok(())
	}
}
