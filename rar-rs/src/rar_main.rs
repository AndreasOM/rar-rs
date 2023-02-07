use clap::Parser;
use oml_game::Game;
use rar_rs::rar::RarApp;
use tracing::*;
use tracing_subscriber::FmtSubscriber;
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
	/// Optional script to run after start
	#[arg(long, value_name = "SCRIPT")]
	script: Option<String>,
}

fn main() -> anyhow::Result<()> {
	println!("RAR!");
	let cli = Cli::parse();

	let use_ansi = atty::is(atty::Stream::Stdout);

	let subscriber = FmtSubscriber::builder()
		.with_max_level(Level::TRACE)
		.with_ansi(use_ansi) // sublime console doesn't like it :(
		.finish();

	tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

	let mut app = RarApp::new();

	if let Some(script) = cli.script.as_deref() {
		println!("Value for script: {}", script);
		app.queue_script(script);
	}

	match Game::run(app) {
		Ok(_) => {},
		Err(e) => {
			error!("Game returned {}", &e)
		},
	}

	Ok(())
}
