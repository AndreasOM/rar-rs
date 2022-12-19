use derive_getters::Getters;
use oml_game::system::System;
use tracing::*;

#[derive(Debug, Default, Getters)]
pub struct Tile {
	id:          u32,
	image:       String,
	imagewidth:  u32,
	imageheight: u32,
}

#[derive(Debug, Default, Getters)]
pub struct Tileset {
	columns:     u32,
	name:        String,
	tilecount:   u32,
	tilewidth:   u32,
	tileheight:  u32,
	tiles:       Vec<Tile>, // :TODO: make hashmap based on id?
	remove_path: bool,
}

impl Tile {
	pub fn without_path(mut self) -> Self {
		let s = if let Some(idx) = self.image.rfind("/") {
			idx + 1
		} else {
			0
		};

		let image = self.image[s..].to_string();
		debug!("Tile: {}", &image);
		self.image = image;
		todo!();
		self
	}
}

impl Tileset {
	pub fn new() -> Self {
		Self {
			..Default::default()
		}
	}

	pub fn enable_remove_path(&mut self) {
		self.remove_path = true;
		debug!("Tileset::enable_remove_path: {:?}", &self);
	}

	pub fn add_tile(&mut self, tile: Tile) {
		// debug!("Tileset::add_tile: {:?}", &self);
		let tile = if self.remove_path {
			tile.without_path()
		} else {
			tile
		};

		self.tiles.push(tile);
	}

	pub fn load(&mut self, system: &mut System, name: &str) -> anyhow::Result<()> {
		//		return anyhow::bail!("Just testing...");

		let tsj_name = format!("{}.tsj", &name);
		dbg!(&tsj_name);
		if system.default_filesystem().exists(&tsj_name) {
			println!("Trying to load tileset from {}", &tsj_name);
			let mut tileset_tsj = TilesetTsj::new();
			tileset_tsj.load(system, &tsj_name)?;

			dbg!(&tileset_tsj);
			if self.remove_path {
				tileset_tsj.remove_paths();
			}

			*self = tileset_tsj.into();
			Ok(())
		} else {
			anyhow::bail!("No remaining loader for tileset: {}", &name);
		}
	}

	pub fn get_tile_image(&self, tid: u32) -> &str {
		if let Some(tile) = self.tiles.iter().find(|&t| t.id == tid) {
			&tile.image
		} else {
			""
		}
	}
}

impl From<&tileset_tsj::Tile> for Tile {
	fn from(ttsj: &tileset_tsj::Tile) -> Self {
		let image = ttsj.image();
		Self {
			id:          *ttsj.id(),
			image:       image.split(".").nth(0).unwrap_or(image).to_owned(),
			imagewidth:  *ttsj.imagewidth(),
			imageheight: *ttsj.imageheight(),
		}
	}
}

impl From<tileset_tsj::TilesetTsj> for Tileset {
	fn from(ttsj: tileset_tsj::TilesetTsj) -> Self {
		let mut ts = Tileset::new();
		ts.name = ttsj.name().to_owned();
		ts.columns = *ttsj.columns();
		ts.tilecount = *ttsj.tilecount();
		ts.tilewidth = *ttsj.tilewidth();
		ts.tileheight = *ttsj.tileheight();

		for t in ttsj.tiles() {
			ts.add_tile(t.into());
		}

		ts
	}
}

#[path = "./tileset_tsj.rs"]
mod tileset_tsj;
use tileset_tsj::TilesetTsj;

#[cfg(test)]
mod tests {
	use oml_game::system::filesystem_disk::FilesystemDisk;
	use oml_game::system::System;

	// use crate::rar::Map;
	use super::*;

	#[test]
	fn tileset_loading_works() -> anyhow::Result<()> {
		let run_test = || -> anyhow::Result<()> {
			let cwd = std::env::current_dir().unwrap();
			let cwd = cwd.to_string_lossy();

			let datadir = format!("{}/../rar-data", &cwd);

			let mut dfs = FilesystemDisk::new(&datadir);

			let mut system = System::new();

			system.set_default_filesystem(Box::new(dfs));

			let mut tileset = Tileset::new();
			tileset.load(&mut system, "rar-default")?;

			dbg!(&tileset);
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
