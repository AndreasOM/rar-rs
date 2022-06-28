use oml_game::system::filesystem_disk::FilesystemDisk;
use oml_game::system::filesystem_layered::FilesystemLayered;
use oml_game::system::System;
use oml_game::window::{Window, WindowUpdateContext};
use oml_game::App;

pub struct RarApp {
	system:  System,
	is_done: bool,
}

impl RarApp {
	pub fn new() -> Self {
		Self {
			system:  System::new(),
			is_done: false,
		}
	}
	// :TODO: Consider moving this into game package
	fn add_filesystem_disk(&mut self, lfs: &mut FilesystemLayered, path: &str, enable_write: bool) {
		let datadir = if path.starts_with("/") {
			path.to_owned()
		} else {
			let cwd = std::env::current_dir().unwrap();
			let cwd = cwd.to_string_lossy();

			let datadir = format!("{}/{}", &cwd, &path);
			datadir
		};

		let mut dfs = FilesystemDisk::new(&datadir);
		if enable_write {
			dfs.enable_write();
		}

		lfs.add_filesystem(Box::new(dfs));
	}
}

impl App for RarApp {
	fn setup(&mut self, window: &mut Window) -> anyhow::Result<()> {
		window.set_title("RAR - RS");

		let mut lfs = FilesystemLayered::new();
		self.add_filesystem_disk(&mut lfs, "../rar-data", false);

		println!("lfs: {:?}", &lfs);

		self.system.set_default_filesystem(Box::new(lfs));


		let mut something_file = self.system.default_filesystem_mut().open( "something.txt" );
//		println!("sf: {:?}", &something_file);
//		println!("valid?: {:?}", something_file.is_valid());
//		println!("size: {:?}", something_file.size());
		let something = something_file.read_as_string();

		println!("Something: {}", &something);

		Ok(())
	}
	fn teardown(&mut self) {}
	fn is_done(&self) -> bool {
		self.is_done
	}
	fn update(&mut self, wuc: &mut WindowUpdateContext) {
		if wuc.is_escape_pressed {
			self.is_done = true;
		}
		if wuc.mouse_buttons[0] {
			println!("{} {}", wuc.mouse_pos.x, wuc.mouse_pos.y);
		}
	}
	fn render(&mut self) {}
}
