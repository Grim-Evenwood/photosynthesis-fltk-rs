use crate::gui::GUI;
mod gui;

/// # main
/// method where program starts
fn main() {
	let mut gui = GUI::default();
	gui.initialize();
    gui.button_grid_test();
	gui.show();
	while gui.app.wait() {
	}
}//end main method
