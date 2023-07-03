use fltk::{window::Window, app::App, prelude::{WidgetExt, GroupExt}};

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
	}
	pub fn show(&mut self){
		self.main_window.show();
	}
}//end impl for GUI