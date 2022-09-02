use oml_game::Game;
use rar_rs::rar::RarApp;
use tracing::*;
use tracing_subscriber::FmtSubscriber;

fn main() -> anyhow::Result<()> {
	println!("RAR!");
	let use_ansi = atty::is(atty::Stream::Stdout);

	let subscriber = FmtSubscriber::builder()
		.with_max_level(Level::TRACE)
		.with_ansi(use_ansi) // sublime console doesn't like it :(
		.finish();

	tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

	let app = RarApp::new();

	match Game::run(app) {
		Ok(_) => {},
		Err(e) => {
			error!("Game returned {}", &e)
		},
	}

	Ok(())
}
