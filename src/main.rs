mod gui;
use crate::gui::GUI;
mod game;
use game::Board;

/// # main
/// method where program starts
fn main() {
	// set up program model
	let game_board = Board::default();

	// set up gui
	let mut gui = GUI::default();
	gui.initialize();
    gui.button_grid_test();

	// display gui and start program
	gui.show();
	while gui.app.wait() {
	}
}//end main method
