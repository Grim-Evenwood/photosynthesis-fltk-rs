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

/// # FlexGrid
/// 
/// This struct is meant to be a sort of wrapper around a bunch of buttons and nested flexes in order to mimic a grid of buttons.
pub struct FlexGrid {
	/// # buttons
	/// The 2d array of buttons filling the grid
	pub buttons: Grid<Button>,
	/// # outer_flex
	/// The flex containing the flex containing the buttons
	pub outer_flex: Flex,
	/// # inner_flexes
	/// the flexes contained within the inner flex
	pub inner_flexes: Vec<Flex>,
}//end struct FlexGrid

impl FlexGrid {
	/// # default()
	/// 
	/// constructs the empty FlexGrid
	pub fn default() -> FlexGrid {
		FlexGrid {
			buttons:Grid::new(0,0),
			outer_flex:Flex::new(0, get_default_menu_height() + get_default_tab_padding(), get_default_grid_width(), get_default_grid_height(), None),
			inner_flexes:Vec::new(),
		}//end struct construction
	}//end new()

	/// # clear_inner_flexes
	/// 
	/// clears the children of this struct. should hopefully work
	pub fn clear_inner_flexes(&mut self) {
		self.outer_flex.clear();
		self.inner_flexes.clear();
		self.buttons.clear();
	}//end clear_inner_flexes(&mut self)

	/// #initialize_flex(self, grid)]
	/// 
	/// Sets up the flex-boxes like a grid
	pub fn initialize_flex(&mut self, rows:usize, cols:usize) {
		// set outer flex to be have rows of elements
		self.outer_flex.set_type(group::FlexType::Column);
		self.outer_flex.set_align(Align::LeftTop);
		for _row_index in 0..rows {
			let inner_flex_x = 0;//self.outer_flex.x();
			let inner_flex_y = self.outer_flex.y() + (self.outer_flex.width() / cols as i32);
			let inner_flex_w = get_default_grid_width() / cols as i32;
			let inner_flex_h = get_default_grid_height() / rows as i32;
			let mut inner_flex = Flex::new(inner_flex_x,inner_flex_y,inner_flex_w,inner_flex_h,None);
			inner_flex.set_type(group::FlexType::Row);
			// make flex show up
			self.outer_flex.add(&inner_flex);
			// save flex to struct
			self.inner_flexes.push(inner_flex);
		}//end adding inner flexes
		// println!("{} inner flexes", self.inner_flexes.len());
		// println!("inner flex x:{}", self.inner_flexes.first().unwrap().x());
	}//end initialize_flex(self, grid)

	/// # fill_flex(self, buttons)
	/// fills up the flex with buttons such that the buttons will show up in the flex looking like a grid
	/// 
	/// It should be noted that this function should expect to receive things in the order of col, rows
	pub fn fill_flex(&mut self, buttons:&Grid<Button>) {
		for row_idx in 0..buttons.rows() {
			let this_inner_flex = self.inner_flexes.get_mut(row_idx).unwrap();
			// loop over the current row of buttons
			for button in buttons.iter_row(row_idx) {
				if !button.was_deleted() {
					this_inner_flex.add(button);
				}//end if button wasn't deleted
				else {println!("button was deleted, row {}", row_idx);}
			}//end adding each button in row to inner flex
		}//end looping over each inner flex and adding buttons
	}//end fill_flex
}//end impl for FlexGrid
