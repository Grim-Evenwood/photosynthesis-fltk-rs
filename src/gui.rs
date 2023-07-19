use fltk::{window::Window, app::{App, Receiver, Sender, self}, prelude::{WidgetExt, GroupExt, WidgetBase, MenuExt, DisplayExt, ImageExt}, enums::{Color, Align, Shortcut}, button::Button, group::{Flex, self}, menu::{SysMenuBar, self, Choice}, text::{TextEditor, TextBuffer, self, TextDisplay}, image::PngImage, frame::Frame};
use grid::Grid;
use fltk_theme::WidgetScheme;
use fltk_theme::SchemeType;
use fltk_theme::widget_themes;
use crate::{Board, game::{Fertility, Tree}};

#[warn(missing_docs)]
pub struct GUI {
	/// application everything runs inside of 
	pub app:App,
	/// main window of application
	pub main_window: Window,
	/// sends messages for events
	pub msg_sender:Sender<String>,
	/// receives messages for events
	pub msg_receiver:Receiver<String>,
	/// menu bar
	pub menu:SysMenuBar,
	/// info bar turn count
	pub turn_count:TextBuffer,
	/// info bar player points 
	pub player_points:TextBuffer,
	/// info bar light points
	pub light_pts:TextBuffer,
	/// info bar lunar points 
	pub lunar_pts:TextBuffer,
	/// UI GUI board display
	pub board_flex_grid:FlexGrid,
	/// menu for buying trees
	pub buying_trees_choice:Choice,
	/// menu for availibe trees
	pub availible_trees:TextBuffer,
	pub available_trees_display:TextDisplay,
}//end struct GUI

impl Default for GUI {
	/// # default()
	/// 
	/// 
	fn default() -> GUI {
		let (s,r) = app::channel();		
		GUI {
			app: App::default(),
			main_window: Window::default(),
			msg_sender: s,
			msg_receiver: r,
			menu:SysMenuBar::default(),
			turn_count:TextBuffer::default(),
			player_points:TextBuffer::default(),
			light_pts:TextBuffer::default(),
			lunar_pts:TextBuffer::default(),
			board_flex_grid:FlexGrid::default(),
			buying_trees_choice:Choice::default(),
			availible_trees:TextBuffer::default(),
			available_trees_display:TextDisplay::default(),
		}//end struct construction
	}//end default()
}//end impl Default for GUI

#[warn(missing_docs)]
impl GUI {
	/// # initialize(self)
	pub fn initialize(&mut self) {
		// Widget Scheme 
		let widget_scheme = WidgetScheme::new(SchemeType::Aqua);
		widget_scheme.apply();

		// window settings
		self.main_window.set_size(get_default_win_width(),get_default_win_height());
		self.main_window.make_resizable(true);
		self.main_window.set_label("uwu Photosynthesis");
		self.main_window.set_label_size(32);
		self.main_window.set_label_color(Color::Green);

		// top menu bar settings
		self.menu.set_size(get_default_win_width(), get_default_menu_height());
		self.menu.set_label_size(10);
		self.menu.set_label_color(Color::Green);

		// Text settings
		let mut txt1 = TextEditor::default()
			.with_size(get_default_txt_width(),get_default_txt_height())
			.with_label("Turn")
			.with_pos(get_default_txt_padding() * 2, self.menu.height() + self.menu.y() + 25);	

		let mut txt2 = TextEditor::default()
			.with_size(get_default_txt_width(),get_default_txt_height())
			.with_label("Player/Computer Pts")
			.right_of(&txt1, get_default_txt_padding());	

		let mut txt3 = TextEditor::default()
			.with_size(get_default_txt_width(),get_default_txt_height())
			.with_label("Light Pts")
			.right_of(&txt2, get_default_txt_padding());	

		let mut txt4 = TextEditor::default()
			.with_size(get_default_txt_width(),get_default_txt_height())
			.with_label("Lunar Pts")
			.right_of(&txt3, get_default_txt_padding());	
		
		let mut txt5 = Button::default()
			.with_size(get_default_txt_width(),get_default_txt_height())
			.with_label("Toggle Plan/Place")
			.right_of(&txt4, get_default_txt_padding());	

		let mut txt6 = Button::default()
			.with_size(get_default_txt_width(),get_default_txt_height())
			.with_label("End Turn")
			.right_of(&txt5, get_default_txt_padding());	

		txt1.set_buffer(self.turn_count.clone());
		txt2.set_buffer(self.player_points.clone());
		txt3.set_buffer(self.light_pts.clone());
		txt4.set_buffer(self.lunar_pts.clone());
		txt5.set_frame(widget_themes::OS_SPACER_THIN_DOWN_BOX);
		txt5.set_color(Color::from_rgb(68,140,184));
		txt6.set_frame(widget_themes::OS_SPACER_THIN_DOWN_BOX);
		txt6.set_color(Color::from_rgb(184,68,68));

		txt1.wrap_mode(text::WrapMode::AtBounds, 0);
		txt2.wrap_mode(text::WrapMode::AtBounds, 0);
		txt3.wrap_mode(text::WrapMode::AtBounds, 0);
		txt4.wrap_mode(text::WrapMode::AtBounds, 0);

		self.main_window.add(&txt1);
		self.main_window.add(&txt2);
		self.main_window.add(&txt3);
		self.main_window.add(&txt4);
		self.main_window.add(&txt5);
		self.main_window.add(&txt6);
	}//end initialize(self)
	
	pub fn initialize_sun(&mut self) {
		let mut frame = Frame::default()
			.with_size(100,100)
			.with_pos(get_default_win_width() - get_default_grid_width() - 150, 125);
		let mut image = PngImage::load("imgs/photosynthesis sun.png").unwrap();
		image.scale(400, 400, true, true);
		frame.set_image(Some(image));

		self.main_window.add(&frame);
	}//end initialize_sun(&mut self)
	

	/// # show(self)
	pub fn show(&mut self){
		self.main_window.show();
	}//end show(self)

	#[allow(dead_code)]
	/// # button_grid_test(self)
	pub fn button_grid_test(&mut self) {
		let mut test_grid: Grid<Button> = Grid::new(5,5);
		let mut flex = FlexGrid::default();
		flex.clear_inner_flexes();

		// initialize test_grid
		for row in 0..test_grid.rows() {
			for col in 0..test_grid.cols() {
				// set settings for button
				let mut new_button = Button::default();
				new_button.set_label(&format!("row:{}, col:{}", row, col));
				new_button.set_size(10, 10);

				if new_button.width() < get_max_grid_button_width() || new_button.height() <  get_max_grid_button_height() {
					new_button.set_label("");
				}//end if button is too small

				// add button click event
				new_button.emit(self.msg_sender.clone(), format!("test:{},{}",row,col));

				// add button into grid space
				let grid_spot = test_grid.get_mut(row, col).unwrap();
				*grid_spot = new_button;
			}//end looping across columns
		}//end looping across rows

		// initialize the flex grid
		flex.initialize_flex(5, 5);
		flex.fill_flex(&test_grid);

		// make the flex actually show up in program
		self.main_window.add(&flex.outer_flex);
		flex.outer_flex.recalc();
	}//end button_grid_test(self)

	/// # initialize_menu(&mut self)
	pub fn initialize_menu(&mut self) {
		self.menu.add_emit(
			"Save\t",
			Shortcut::Ctrl | 's',
			menu::MenuFlag::Normal,
			self.msg_sender.clone(),
			"MenuChoice::Save".to_string(),
		);
		
		self.menu.add_emit(
			"Load\t",
			Shortcut::Ctrl | 'n',
			menu::MenuFlag::Normal,
			self.msg_sender.clone(),
			"MenuChoice::Save".to_string(),
		);

		self.menu.add_emit(
			"Change Season/Winter\t",
			Shortcut::None,
			menu::MenuFlag::Normal,
			self.msg_sender.clone(),
			"MenuChoice::Save".to_string(),
		);
		
		self.menu.add_emit(
			"Change Season/Spring\t",
			Shortcut::None,
			menu::MenuFlag::Normal,
			self.msg_sender.clone(),
			"MenuChoice::Save".to_string(),
		);

		self.menu.add_emit(
			"Change Season/Summer\t",
			Shortcut::None,
			menu::MenuFlag::Normal,
			self.msg_sender.clone(),
			"MenuChoice::Save".to_string(),
		);

		self.menu.add_emit(
			"Change Season/Fall\t",
			Shortcut::None,
			menu::MenuFlag::Normal,
			self.msg_sender.clone(),
			"MenuChoice::Save".to_string(),
		);

		self.main_window.add(&self.menu);
	}

	/// # initialize_board(&mut self, board)
	/// 
	/// This function adds the game board to the main window so it can be displayed.  
	/// 
	/// It also takes in the board state so it can populate that display with the initial state of the game board.
	/// 
	/// ## parameters
	/// board_state : Reference to the object holding the current state of the board.
	/// 
	/// ## result
	/// Board is displayed in the main window, showing the current state of the board.
	/// 
	pub fn initialize_board(&mut self, board_state:&Board) {
		/*
		Maybe look to button_grid_test for example of creating grid of buttons
		and displaying it in a FlexGrid.
		*/

		let mut board: Grid<Button> = Grid::new(board_state.board.rows(),board_state.board.cols());
		self.board_flex_grid = FlexGrid::default();
		self.board_flex_grid.clear_inner_flexes();
		self.board_flex_grid.outer_flex.clear();
		
		// intiailize board 
		for row in 0..board.rows(){
			for col in 0..board.cols(){
				//settings for button
				let mut new_button = Button::default();
				new_button.set_label(&format!("row:{}\ncol:{}", row, col));
				new_button.set_size(get_default_grid_width() / board.cols() as i32, get_default_grid_height() / board.rows() as i32);

				if new_button.width() < get_max_grid_button_width() || new_button.height() < get_max_grid_button_height() {
					new_button.set_label("");
				}// end if button is too small
				
				// fertility button colors
				new_button.set_color(Color::from_rgb(50, 168, 82));
				let this_board_spot = board_state.board.get(row, col).unwrap();
				match this_board_spot.fertility {
					Fertility::OneLeaf => new_button.set_color(Color::from_rgb(147, 196, 125)),
					Fertility::TwoLeaf => new_button.set_color(Color::from_rgb(106, 168, 79)),
					Fertility::ThreeLeaf => new_button.set_color(Color::from_rgb(56, 118, 29)),
					Fertility::FourLeaf => new_button.set_color(Color::from_rgb(39, 78, 19)),
				}

				// add buttton click event
				new_button.emit(self.msg_sender.clone(), format!("uwu board:{},{}", row, col));

				// add button into grid space
				let grid_spot = board.get_mut(row, col).unwrap();
				*grid_spot = new_button;
			}// end looping for columns
		}// end looping for rows 

		// initialize flex grid
		self.board_flex_grid.initialize_flex(board_state.board.rows(),board_state.board.cols());
		self.board_flex_grid.fill_flex(&board);

		// make flex show up 
		self.board_flex_grid.outer_flex.set_pos(get_default_win_width() - get_default_grid_width() - 100, 175);
		self.main_window.add(&self.board_flex_grid.outer_flex);
		self.board_flex_grid.outer_flex.recalc();
	}//end initialize_board(&mut self, board)

	/// # initialize_tree_lists(self, to_buy, available)
	/// 
	/// Takes list of trees which are to buy or available and initializes menu elements and lists.
	pub fn initialize_tree_lists(&mut self, to_buy: Vec<Tree>, available: Vec<Tree>) {
		// action buttons
		self.buying_trees_choice = Choice::default()
			.with_size(135,30)
			.with_pos(100, 175)
			.with_label("Items to buy");
		self.update_tree_lists(to_buy, available);
		self.buying_trees_choice.set_color(Color::from_rgb(56, 118, 29));
		self.buying_trees_choice.set_text_color(Color::White);
		self.main_window.add(&self.buying_trees_choice);

		self.available_trees_display = TextDisplay::default()
			.with_size(135,150)
			.below_of(&self.buying_trees_choice,100)
			.with_label("Available Area")
			.with_align(Align::LeftTop);

		self.available_trees_display.set_buffer(self.availible_trees.clone());
		self.available_trees_display.set_color(Color::from_rgb(147, 196, 125))
	}//end initialize_tree_lists(self, to_buy, available)

	pub fn update_tree_lists(&mut self, to_buy: Vec<Tree>, available: Vec<Tree>) {
		// clear any previous options
		self.buying_trees_choice.clear();

		let mut num_seeds = 0;
		let mut num_small_trees = 0;
		let mut num_med_trees = 0;
		let mut num_large_trees = 0;
		
		for tree in to_buy {
			match tree.size {
				crate::game::TreeSize::Seed => num_seeds += 1,
				crate::game::TreeSize::Small => num_small_trees += 1,
				crate::game::TreeSize::Medium => num_med_trees += 1,
				crate::game::TreeSize::Large => num_large_trees += 1
			}
		}

		self.buying_trees_choice.add_emit(
			format!("Seed ({})", num_seeds).as_str(),
			Shortcut::None,
			menu::MenuFlag::Normal,
			self.msg_sender.clone(),
			"Buy:Seed".to_string()
		);

		self.buying_trees_choice.add_emit(
			format!("Small Tree ({})", num_small_trees).as_str(), 
			Shortcut::None, 
			menu::MenuFlag::Normal, 
			self.msg_sender.clone(),
			"Buy:Small".to_string()
		);

		self.buying_trees_choice.add_emit(
			format!("Medium Tree ({})", num_med_trees).as_str(),
			Shortcut::None,
			menu::MenuFlag::Normal,
			self.msg_sender.clone(),
			"Buy:Medium".to_string()
		);

		self.buying_trees_choice.add_emit(
			format!("Large Tree ({})", num_large_trees).as_str(),
			Shortcut::None,
			menu::MenuFlag::Normal,
			self.msg_sender.clone(),
			"Buy:Large".to_string()
		);

		let mut available_text = "".to_string();
		for avai in available {
			available_text += format!("{}\n", avai.size).as_str();
		}//end looping over available trees
		self.availible_trees.set_text(&available_text);
	}//end update_tree_lists(self, to_buy, available)
}//end impl for GUI

fn get_default_win_width() -> i32 {1000}
fn get_default_win_height() -> i32 {700}
fn get_default_menu_height() -> i32 {30}
fn get_default_tab_padding() -> i32 {0}
fn get_default_grid_width() -> i32 {get_default_win_width() - 400}
fn get_default_grid_height() -> i32 {get_default_win_height()-get_default_menu_height() - get_default_tab_padding() - 225}
fn get_max_grid_button_width() -> i32 {30}
fn get_max_grid_button_height() -> i32 {15}
fn get_default_txt_width() -> i32 {(get_default_win_width() / 6) - (get_default_txt_padding() * 3 / 2)}
fn get_default_txt_height() -> i32 {25}
fn get_default_txt_padding() -> i32 {10}

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
		let new_outer_flex = Flex::new(0, get_default_menu_height() + get_default_tab_padding(), get_default_grid_width(), get_default_grid_height(), None);
		new_outer_flex.end();
		FlexGrid {
			buttons: Grid::new(0,0),
			outer_flex: new_outer_flex,
			inner_flexes: Vec::new(),
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
			this_inner_flex.end();
		}//end looping over each inner flex and adding buttons
		self.outer_flex.end();
	}//end fill_flex
}//end impl for FlexGrid