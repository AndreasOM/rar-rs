use oml_game::system::System;

#[derive(Debug)]
pub struct Map {}

impl Map {
	fn new() -> Self {
		Self {}
	}

	fn load(&mut self, system: &mut System, name: &str) -> anyhow::Result<()> {
		//		return anyhow::bail!("Just testing...");

		let tmj_name = format!("{}.tmj", &name);
		dbg!(&tmj_name);
		if system.default_filesystem().exists(&tmj_name) {
			println!("Trying to load map from {}", &tmj_name);
			let mut map_tmj = MapTmj::new();
			map_tmj.load(system, &tmj_name)?;

			dbg!(&map_tmj);
		} else {
			return anyhow::bail!("No remaining loader for map: {}", &name);
		}
		Ok(())
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
