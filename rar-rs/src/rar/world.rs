use derive_getters::Getters;
use oml_game::system::System;

use crate::rar::Map;

pub const UPSIDEUP: bool = true;

#[derive(Debug, Default, Getters)]
pub struct WorldMap {
	filename: String, // prefix!
	x:        u32,
	y:        u32,
	height:   u32,
	width:    u32,
	map:      Option<Map>,
}

#[derive(Debug, Default, Getters)]
pub struct World {
	maps: Vec<WorldMap>,
}

impl World {
	pub fn new() -> Self {
		Self { maps: Vec::new() }
	}

	fn add_map(&mut self, map: WorldMap) {
		self.maps.push(map);
	}

	pub fn load_all_maps(&mut self, system: &mut System) -> anyhow::Result<()> {
		for m in self.maps.iter_mut() {
			let mut map = Map::new();
			map.load(system, &m.filename)?;

			if *map.upsideup() != UPSIDEUP {
				map.hflip(512.0);
			}

			m.map = Some(map);
		}
		Ok(())
	}

	pub fn load(&mut self, system: &mut System, name: &str) -> anyhow::Result<()> {
		//		return anyhow::bail!("Just testing...");

		let world_name = format!("{}.world", &name);
		dbg!(&world_name);
		if system.default_filesystem().exists(&world_name) {
			println!("Trying to load world from {}", &world_name);
			let mut world_world = WorldWorld::new();
			world_world.load(system, &world_name)?;

			dbg!(&world_world);

			*self = world_world.into();
		} else {
			return anyhow::bail!("No remaining loader for world: {}", &name);
		}
		Ok(())
	}
}

impl From<WorldWorld> for World {
	fn from(ww: WorldWorld) -> Self {
		let mut w = World::default();
		for wwm in ww.maps().iter() {
			let m = wwm.into();

			w.add_map(m);
		}
		w
	}
}

impl From<&world_world::Map> for WorldMap {
	fn from(wwm: &world_world::Map) -> Self {
		Self {
			x:        wwm.x(),
			y:        wwm.y(),
			width:    wwm.width(),
			height:   wwm.height(),
			filename: wwm.filename().strip_suffix(".tmj").unwrap_or("").to_owned(), // :TODO: strip suffix
			map:      None,
		}
	}
}

#[path = "./world_world.rs"]
mod world_world;
use world_world::WorldWorld;

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

			let mut world = World::new();
			world.load(&mut system, "dev")?;
			world.load_all_maps(&mut system)?;
			dbg!(&world);
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