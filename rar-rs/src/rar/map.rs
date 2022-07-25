use derive_getters::Getters;
use oml_game::system::System;

/* we could use an enum for the different layer types, but for now we just mix into on struct?!
#[derive(Debug)]
enum Layer {
	Objects
}
*/
#[derive(Debug, Default)]
pub enum LayerType {
	Objects,
	#[default]
	None,
}

#[derive(Debug, Default)]
pub enum ObjectData {
	Rectangle {
		x:      f64,
		y:      f64,
		width:  f64,
		height: f64,
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

#[derive(Debug, Default, Getters)]
pub struct Layer {
	layertype: LayerType,
	name:      String,
	objects:   Vec<Object>,
}

impl Layer {
	pub fn add_object(&mut self, object: Object) {
		self.objects.push(object);
	}
}

#[derive(Debug, Default, Getters)]
pub struct Map {
	layers: Vec<Layer>,
}

impl Map {
	pub fn new() -> Self {
		Self {
			..Default::default()
		}
	}

	pub fn add_layer(&mut self, layer: Layer) {
		self.layers.push(layer);
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
			return anyhow::bail!("No remaining loader for map: {}", &name);
		}
		Ok(())
	}
}

impl From<&map_tmj::Object> for Object {
	fn from(otmj: &map_tmj::Object) -> Self {
		let data = if otmj.point() {
			ObjectData::Unknown
		} else {
			ObjectData::Rectangle {
				x:      otmj.x(),
				y:      otmj.y(),
				width:  otmj.width(),
				height: otmj.height(),
			}
		};

		Self {
			name: otmj.name().to_owned(),
			class: otmj.class().to_owned(),
			data,
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
			_ => LayerType::None,
		};

		l
	}
}

impl From<map_tmj::MapTmj> for Map {
	fn from(mtmj: map_tmj::MapTmj) -> Self {
		let mut m = Map::new();
		for l in mtmj.layers() {
			m.add_layer(l.into());
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
