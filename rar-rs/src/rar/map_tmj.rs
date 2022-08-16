use derive_getters::Getters;
use oml_game::system::System;
//use serde_json::Result;
use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};

use crate::rar::map::TileMap;

#[derive(Debug, Default, Getters, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Tileset {
	firstgid: u32,
	source:   String,
	//	#[serde(rename = "type")]
	//	tilesettype: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Object {
	name:     String,
	class:    String,
	id:       u32,
	x:        f64,
	y:        f64,
	height:   f64,
	width:    f64,
	rotation: f64, // deg, clockwise
	visible:  bool,
	#[serde(default)]
	point:    bool,
}

impl Object {
	pub fn name(&self) -> &str {
		&self.name
	}
	pub fn class(&self) -> &str {
		&self.class
	}
	pub fn x(&self) -> f64 {
		self.x
	}
	pub fn y(&self) -> f64 {
		self.y
	}
	pub fn height(&self) -> f64 {
		self.height
	}
	pub fn width(&self) -> f64 {
		self.width
	}
	pub fn point(&self) -> bool {
		self.point
	}
}

#[derive(Debug, Default, Getters, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Chunk {
	data:   String,
	x:      i32,
	y:      i32,
	height: u32,
	width:  u32,
	#[serde(skip)]
	tiles:  TileMap, //Vec< u32 >,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Layer {
	name:        String,
	id:          u32,
	chunks:      Option<Vec<Chunk>>,
	objects:     Option<Vec<Object>>,
	#[serde(default)]
	compression: String,
	#[serde(default)]
	encoding:    String,
	x:           u32,
	y:           u32,
	#[serde(default)]
	height:      u32,
	#[serde(default)]
	width:       u32,
	#[serde(default)]
	startx:      i32,
	#[serde(default)]
	starty:      i32,
	opacity:     f64,
	#[serde(rename = "type")]
	layertype:   String,
	visible:     bool,
	#[serde(default)]
	draworder:   String,
}

impl Layer {
	pub fn name(&self) -> &str {
		&self.name
	}
	pub fn id(&self) -> u32 {
		self.id
	}
	pub fn layertype(&self) -> &str {
		&self.layertype
	}
	pub fn visible(&self) -> bool {
		self.visible
	}
	pub fn x(&self) -> u32 {
		self.x
	}
	pub fn y(&self) -> u32 {
		self.y
	}
	pub fn height(&self) -> u32 {
		self.height
	}
	pub fn width(&self) -> u32 {
		self.width
	}
	pub fn chunks(&self) -> &Option<Vec<Chunk>> {
		&self.chunks
	}
	pub fn objects(&self) -> &Option<Vec<Object>> {
		&self.objects
	}
}
#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MapTmj {
	layers:           Vec<Layer>,
	tileheight:       u32,
	tilewidth:        u32,
	compressionlevel: i32,
	width:            u32,
	height:           u32,
	infinite:         bool,
	nextlayerid:      u32,
	nextobjectid:     u32,
	orientation:      String,
	renderorder:      String,
	tiledversion:     String,
	tilesets:         Vec<Tileset>,
	#[serde(rename = "type")]
	maptype:          String,
	version:          String,
}

impl MapTmj {
	pub fn new() -> Self {
		Self {
			layers: Vec::new(),
			tileheight: 0,
			tilewidth: 0,
			..Default::default()
		}
	}

	pub fn tileheight(&self) -> u32 {
		self.tileheight
	}

	pub fn tilewidth(&self) -> u32 {
		self.tilewidth
	}

	pub fn layers(&self) -> &Vec<Layer> {
		&self.layers
	}

	pub fn tilesets(&self) -> &Vec<Tileset> {
		&self.tilesets
	}

	fn decode_chunks(&mut self) -> anyhow::Result<()> {
		/*
		The base64-encoded and optionally compressed layer data is somewhat more complicated to parse. First you need to base64-decode it, then you may need to decompress it. Now you have an array of bytes, which should be interpreted as an array of unsigned 32-bit integers using little-endian byte ordering.

		Whatever format you choose for your layer data, you will always end up with so called “Global Tile IDs” (gids). They are called “global”, since they may refer to a tile from any of the tilesets used by the map. The IDs also contain flipping flags. The tilesets are always stored with increasing firstgids.
		*/
		for l in self.layers.iter_mut() {
			if let Some(chunks) = &mut l.chunks {
				for c in chunks.iter_mut() {
					if l.encoding != "base64" {
						return anyhow::bail!("Non base64 layer encoding not supported!");
					}
					if l.compression != "" {
						return anyhow::bail!("Compressed layers not supported!");
					}
					let l = c.data.len() / 4;
					let mut v = TileMap::new(c.width, c.height); //Vec::with_capacity(l);

					let data = base64::decode(&c.data)?;
					let mut i = 0;
					for bytes in data.chunks(4) {
						let t = u32::from_le_bytes(bytes[0..4].try_into()?);
						print!("{:08X?} ", &t);
						i += 1;
						if i % c.width == 0 {
							println!("");
						}
						v.push(t);
					}
					c.tiles = v;

					//					dbg!(&c.tiles);
				}
			}
		}
		Ok(())
	}

	pub fn load(&mut self, system: &mut System, name: &str) -> anyhow::Result<()> {
		let mut tmj_file = system.default_filesystem_mut().open(&name);
		let tmj_string = tmj_file.read_as_string();

		let v: Value = serde_json::from_str(&tmj_string)?;
		//		dbg!(&v);

		let tmj: MapTmj = serde_json::from_str(&tmj_string)?;
		//		dbg!(&tmj);

		*self = tmj;
		self.decode_chunks()?;
		Ok(())
	}
}
