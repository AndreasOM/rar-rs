use oml_game::system::System;
//use serde_json::Result;
use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Map {
	#[serde(rename = "fileName")]
	filename: String,
	x:        u32,
	y:        u32,
	height:   u32,
	width:    u32,
}

impl Map {
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
	pub fn filename(&self) -> &str {
		&self.filename
	}
}

#[derive(Debug, Serialize, Deserialize)]
//#[serde(deny_unknown_fields)]
pub struct WorldWorld {
	//	layers: Vec<Layer>,
	#[serde(rename = "type")]
	worldtype:            String,
	onlyShowAdjacentMaps: bool,
	maps:                 Vec<Map>,
}

impl WorldWorld {
	pub fn new() -> Self {
		Self {
			worldtype:            String::default(),
			onlyShowAdjacentMaps: false,
			maps:                 Vec::new(),
		}
	}

	pub fn maps(&self) -> &Vec<Map> {
		&self.maps
	}

	pub fn load(&mut self, system: &mut System, name: &str) -> anyhow::Result<()> {
		let mut world_file = system.default_filesystem_mut().open(&name);
		let world_string = world_file.read_as_string();

		let v: Value = serde_json::from_str(&world_string)?;
		dbg!(&v);

		let world: WorldWorld = serde_json::from_str(&world_string)?;
		dbg!(&world);

		*self = world;
		Ok(())
	}
}
