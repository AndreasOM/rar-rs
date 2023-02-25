// just a simple test script to test minimal functionality
fn run() {
	debug("Started");
	wait_frames( 10 );
	debug("-> Grassland");
	ui_click_element_with_name( "grassland" );
	ui_click_pos( -400, 128 );
	wait_frames( 10 );
	queue_screenshot( "01-grassland" );			// this is just a suffix
	quit_game();

	debug("-> Mystic Mountain");
	ui_click_element_with_name( "mystic_mountain" );
	ui_click_pos( 0, 128 );
	wait_frames( 10 );
	queue_screenshot( "02-mystic_mountain" );			// this is just a suffix
	quit_game();

	debug("-> Dev");
	ui_click_element_with_name( "dev" );
	ui_click_pos( 400, 128 );
	wait_frames( 10 );
	queue_screenshot( "03-dev" );			// this is just a suffix
	quit_game();

	quit_app();
	//app_quit();
	wait_frames( 3000 );
}

fn quit_game() { // quits the game, not the app!
	debug("Quit Game");
	wait_frames( 10 );
	ui_click_pos( -965, 450 ); // pause
	wait_frames( 30 );
	ui_click_pos( -965, 270 ); // quit game
	wait_frames( 30 );
	ui_click_pos( -880, 270 ); // confirm
	wait_frames( 30 );
}

fn quit_app() { // quits the app, from main menu
	debug("Quit App");
	wait_frames( 10 );
	ui_click_pos( 30, -220 ); // quit app
	wait_frames( 120 );
	ui_click_pos( 35, -60 ); // confirm
	wait_frames( 30 );
}
