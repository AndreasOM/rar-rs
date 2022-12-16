use derive_getters::Getters;
use oml_game::system::System;
//use serde_json::Result;
use serde::{Deserialize, Serialize};
//use serde_json::{Result, Value};
use tracing::*;

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Grid {
	width:       u32,
	height:      u32,
	orientation: String,
}

#[derive(Debug, Default, Getters, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Tile {
	id:          u32,
	image:       String,
	imagewidth:  u32,
	imageheight: u32,
}

#[derive(Debug, Default, Getters, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TilesetTsj {
	columns:      u32,
	grid:         Option<Grid>,
	margin:       u32,
	name:         String,
	spacing:      u32,
	tilecount:    u32,
	tiledversion: String,
	tilewidth:    u32,
	tileheight:   u32,
	#[serde(rename = "type")]
	tilesettype:  String,
	version:      String,
	tiles:        Vec<Tile>,
}

impl Tile {
	pub fn remove_path(&mut self) {
		let s = if let Some(idx) = self.image.rfind("/") {
			idx + 1
		} else {
			0
		};

		let image = self.image[s..].to_string();
		debug!("Tile: {}", &image);
		self.image = image;
	}
}
impl TilesetTsj {
	pub fn new() -> Self {
		Self {
			..Default::default()
		}
	}
	pub fn load(&mut self, system: &mut System, name: &str) -> anyhow::Result<()> {
		let mut tsj_file = system.default_filesystem_mut().open(&name);
		let tsj_string = tsj_file.read_as_string();

		// let v: Value = serde_json::from_str(&tsj_string)?;
		//		dbg!(&v);

		let tsj: TilesetTsj = serde_json::from_str(&tsj_string)?;
		//		dbg!(&tmj);

		*self = tsj;

		// decode here if needed

		Ok(())
	}

	pub fn remove_paths(&mut self) {
		for t in self.tiles.iter_mut() {
			t.remove_path();
		}
	}
}
