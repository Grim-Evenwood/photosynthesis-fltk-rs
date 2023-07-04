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
		if let Some(val) = gui.msg_receiver.recv() {
			// we'll need to figure out what the message was
			if val.starts_with("test") {
				/*
				If the prefix is test, then the msg should be formatted like "test:row,col"
				As such, we want to break off the "test:" part, and then get the row and col number to print it.
				*/
				let colon_split: Vec<&str> = val.split(':').collect();
				/*
				At this point, colon split, should contain two elements: "test" and "row,col".
				We don't care about the "test" because we already used it. Now we just want to look at "row,col",
				in which row and col are numbers representing the row and column number.
				*/
				let colon_split_second = colon_split.get(1).unwrap().to_owned().to_owned();
				let comma_split: Vec<&str> = colon_split_second.split(',').collect();
				let comma_split_first: usize = comma_split.get(0).unwrap().parse().unwrap();
				let comma_split_second: usize = comma_split.get(1).unwrap().parse().unwrap();
				println!("Received test msg from the test button grid. Row:{}, Col:{}", comma_split_first, comma_split_second);
			}//end if ffirst four chars are "test"
		}//end if we got a message
	}//end application loop
}//end main method
