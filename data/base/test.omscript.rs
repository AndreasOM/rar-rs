// just a simple test script to test minimal functionality
fn run() {
	wait_frames( 10 );
	ui_click_element_with_name( "Grassland" );
	wait_frames( 10 );
	queue_screenshot( "grassland" );			// this is just a suffix
	wait_frames( 10 );
	app_quit();
}
