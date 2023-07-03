use fltk::{window::Window, app::App, prelude::{WidgetExt, GroupExt}, enums::Color};

pub struct GUI {
	/// main window of application 
	pub app:App,
	/// 
	pub main_window: Window, 
}

impl Default for GUI {
	/// # default()
	/// 
	/// 
	fn default() -> GUI {
		GUI {
			app: App::default(),
			main_window: Window::default(),
		}//end struct construction
	}//end default()
}//end impl Default for GUI

impl GUI {
	pub fn initialize(&mut self) {
		self.main_window.set_size(500,500);
		self.main_window.make_resizable(true);
		self.main_window.set_label("uwu Photosynthesis");
		self.main_window.set_label_size(32);
		self.main_window.set_label_color(Color::Green);
	}
	pub fn show(&mut self){
		self.main_window.show();
	}
}//end impl for GUI