
use oml_game::App;
use oml_game::window::WindowUpdateContext;

pub struct RarApp {
	is_done: bool,
}

impl RarApp {
	pub fn new() -> Self {
		Self {
			is_done: false,
		}
	}	
}

impl App for RarApp {
	fn setup( &mut self, _window: &mut Window ) -> anyhow::Result<()> {
		Ok(())
	}
	fn teardown( &mut self ) {
	}
	fn is_done( &self ) -> bool {
		self.is_done
	}
	fn update( &mut self, wuc: &mut WindowUpdateContext ) {
		if wuc.is_escape_pressed {
			self.is_done = true;
		}
		if wuc.mouse_buttons[ 0 ] {
			println!("{} {}", wuc.mouse_pos.x, wuc.mouse_pos.y );
		}
	}
	fn render( &mut self ) {
	}	
}