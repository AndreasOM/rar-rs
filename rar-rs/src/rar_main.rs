
use oml_game::Game;

use rar_rs::rar::RarApp;

fn main() -> anyhow::Result< () > {
	println!("RAR!");
	let app = RarApp::new();

	Game::run( app )?;

	Ok(())
}
