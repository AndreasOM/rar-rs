// just a simple test script to test minimal functionality
fn run() {
	wait_frames( 10 );
	ui_click_element_with_name( "grassland" );
	ui_click_pos( -400, 128 );
	wait_frames( 10 );
	queue_screenshot( "grassland" );			// this is just a suffix
	wait_frames( 300 );
	ui_click_pos( -965, 450 ); // pause
	wait_frames( 300 );
	ui_click_pos( -965, 270 ); // quit game
	wait_frames( 300 );
	ui_click_pos( -880, 270 ); // confirm
	wait_frames( 300 );
	ui_click_pos( 30, -220 ); // quit app
	wait_frames( 300 );
	ui_click_pos( 35, -60 ); // confirm
	wait_frames( 300 );
	//app_quit();
}
