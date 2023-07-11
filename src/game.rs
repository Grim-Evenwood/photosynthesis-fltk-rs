use grid::Grid;
use self::enums::{PieceType, Animal, TreeSize, Fertility, SunDirection, MoonDirection, MovingLightDirection};
mod enums;

#[derive(Debug, PartialEq, Eq, Copy, Clone, Ord, PartialOrd, Hash)]
pub struct Sun {
	direction: SunDirection,
}//end struct Sun

#[allow(dead_code)]
impl Sun {
	/// # new(direction)
	/// 
	/// Instantiates a new Sun object with a given direction to start.
	pub fn new(direction: SunDirection) -> Sun {
		Sun {
			direction,
		}//end struct construction
	}//end new()

	/// # next(&self)
	/// 
	/// This function calculates the next direction that the sun will point.  
	/// 
	/// Since the Sun is on one side at a time, this is mostly just a change in direction.
	pub fn next(&self) -> Sun {
		Sun {
			direction: self.direction.next(),
		}//end struct construction
	}//end next(&self)
}//end impl for Sun

impl Default for Sun {
    fn default() -> Self {
        Self {
			direction: SunDirection::Northeast,
		}//end struct construction
    }//end default()
}//end impl Default for Sun

/// # Moon
/// 
/// This object is used to hold information on the location and direction of the Moon.  
/// 
/// The moon sort of sits in between two spots on a grid, and it shines diagonally in two direction.  
/// 
/// For this reason, the moon holds two coordinates, and sits between them.  
/// 
/// It also has a handy function to calculate the next position.  
#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq, Copy, Clone, Ord, PartialOrd, Hash)]
pub struct Moon {
	pub direction: MoonDirection,
	pub row1: usize,
	pub col1: usize,
	pub row2: usize,
	pub col2: usize,
	pub full_moon: bool,
	grid_side_length: usize,
}//end struct Moon

#[allow(dead_code)]
impl Moon {
	/// # new(grid_side_length)
	/// 
	/// This function creates a new moon object.  
	/// 
	/// Moon objects are used with square grids, and you must provide the length of each side of that square grid, in terms of rows/cols.  
	/// 
	/// The moon will start out pointing south, near the upper top right corner of the board.  
	/// 
	/// This function will fail if the side length provided is less than 2.  
	/// 
	pub fn new(grid_side_length: usize) -> Moon {
		Moon {
			direction: MoonDirection::South,
			row1: 0,
			row2: 0,
			col1: grid_side_length - 2,
			col2: grid_side_length - 1,
			full_moon: false,
			grid_side_length,
		}//end struct construction.
	}//end new(grid_side_length)

	/// # next(&self)
	/// 
	/// This function calculates the next position of this moon object, based on the given grid side length.
	/// Please note that this function does not take into account the transition between full and half moon.  
	/// 
	/// 	: When the moon passes the sun, it flips between half and full moon.   
	/// 	: Half moon light gives 1 lunar point, and full moon light gives 2 lunar points.  
	pub fn next(&self) -> Moon {
		// this counter will be used to help us move things.
		let mut spaces_left_to_move = self.grid_side_length + 2;

		// reference variables to build a new moon
		let mut new_row1 = self.row1;
		let mut new_col1 = self.col1;
		let mut new_row2 = self.row2;
		let mut new_col2 = self.col2;
		let mut new_direction = self.direction;
		
		// loop will help us 
		while spaces_left_to_move > 0 {
			match self.direction {
				MoonDirection::South => {
					// how many space can we move counter-clockwise?
					if spaces_left_to_move > new_col1 {
						// space after column
						let spaces_used: usize = (spaces_left_to_move - new_col1) + 1;
						// set columns to 0 (left/west side now)
						new_col1 = 0;
						new_col2 = 0;
						// update row to be first possible value
						new_row1 = 1;
						new_row2 = 0;
						// update direction since we're flipping to a different side of the grid
						new_direction = new_direction.next();
						// update_spaces_left_to_move, since we've used part of it
						spaces_left_to_move -= spaces_used;
					}//end if we'll be changing direction
					else {
						// take spaces left to move out of column location
						new_col1 -= spaces_left_to_move;
						new_col2 -= spaces_left_to_move;
						spaces_left_to_move = 0;
					}//end else we will stay on the same direction
				},
				MoonDirection::East => {
					// how many space can we move counter-clockwise?
					if spaces_left_to_move + new_row1 >= self.grid_side_length  {
						// space after column
						let spaces_used: usize = self.grid_side_length - new_row1;
						// set columns to 0 (left/west side now)
						new_col1 = 1;
						new_col2 = 0;
						// update row to be first possible value
						new_row1 = self.grid_side_length - 1;
						new_row2 = self.grid_side_length - 1;
						// update direction since we're flipping to a different side of the grid
						new_direction = new_direction.next();
						// update_spaces_left_to_move, since we've used part of it
						spaces_left_to_move -= spaces_used;
					}//end if we'll be changing direction
					else {
						// take spaces left to move out of column location
						new_col1 -= spaces_left_to_move;
						new_col2 -= spaces_left_to_move;
						spaces_left_to_move = 0;
					}//end else we will stay on the same direction
				},
				MoonDirection::North => {
					// how many space can we move counter-clockwise?
					if new_col1 + spaces_left_to_move + 2 >= self.grid_side_length {
						// space after column
						let spaces_used: usize = self.grid_side_length - new_col2;
						// set columns to far right side
						new_col1 = self.grid_side_length - 2;
						new_col2 = self.grid_side_length - 2;
						// update row to be first possible value
						new_row1 = self.grid_side_length - 1;
						new_row2 = self.grid_side_length - 2;
						// update direction since we're flipping to a different side of the grid
						new_direction = new_direction.next();
						// update_spaces_left_to_move, since we've used part of it
						spaces_left_to_move -= spaces_used;
					}//end if we'll be changing direction
					else {
						// take spaces left to move out of column location
						new_col1 += spaces_left_to_move;
						new_col2 += spaces_left_to_move;
						spaces_left_to_move = 0;
					}//end else we will stay on the same direction
				},
				MoonDirection::West => {
					// how many space can we move counter-clockwise?
					if spaces_left_to_move > new_row1 {
						// space after column
						let spaces_used: usize = (spaces_left_to_move - new_row1) + 1;
						// set columns to first possible value
						new_col1 = self.grid_side_length-2;
						new_col2 = self.grid_side_length-1;
						// update row to be top side now
						new_row1 = 0;
						new_row2 = 0;
						// update direction since we're flipping to a different side of the grid
						new_direction = new_direction.next();
						// update_spaces_left_to_move, since we've used part of it
						spaces_left_to_move -= spaces_used;
					}//end if we'll be changing direction
					else {
						// take spaces left to move out of column location
						new_row1 -= spaces_left_to_move;
						new_row2 -= spaces_left_to_move;
						spaces_left_to_move = 0;
					}//end else we will stay on the same direction
				},
			}//end matching direction
		}//end looping while we have more spaces to move
		
		Moon {
			direction: new_direction,
			row1: new_row1,
			col1: new_col1,
			row2: new_row2,
			col2: new_col2,
			full_moon: self.full_moon,
			grid_side_length: self.grid_side_length,
		}//end struct construction
	}//end next(&self)
}//end impl for Moon

/// # Board struct
/// 
/// This struct stores the state for the whole game board.  
/// 
/// It stores position in a Grid of BoardSpot objects,
/// and each BoardSpot object stores mechanic information about that position.
/// 
/// It should be noted that several methods will assume that the board is a grid of 7x7, and they will not work without that assumption.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Board {
	/// the grid that represents the game board
	pub board:Grid<BoardSpot>,
	/// the object representing the sun
	pub sun: Sun,
	/// the object representing the moon
	pub moon: Moon,
}//end struct Board

#[allow(dead_code)]
impl Board {
	/// # initialize_board(&mut self)
	/// 
	/// This function will fill the board with appropriate board pieces for the beginning of the game.  
	/// 
	/// This function assumes that the board is a grid of 7x7.
	pub fn initialize_board(&mut self) {
		// create reference pieces to fill board with
		let one_leaf = BoardSpot::new(Fertility::OneLeaf);
		let two_leaf = BoardSpot::new(Fertility::TwoLeaf);
		let three_leaf = BoardSpot::new(Fertility::ThreeLeaf);
		let four_leaf = BoardSpot::new(Fertility::FourLeaf);

		// fill whole board with four_leaf (will end up being at center)
		self.board.fill(four_leaf);

		// overwrite row index 2 and 4 with three leaf
		for piece in self.board.iter_row_mut(2) {
			*piece = three_leaf;
		}//end looping over row index 2
		for piece in self.board.iter_row_mut(4) {
			*piece = three_leaf;
		}//end looping over row index 4

		// overwrite col index 2 and 4 with three leaf
		for piece in self.board.iter_col_mut(2) {
			*piece = three_leaf;
		}//end looping over col index 2
		for piece in self.board.iter_col_mut(4) {
			*piece = three_leaf;
		}//end looping over col index 4

		// overwrite row index 1 and 5 with two leaf
		for piece in self.board.iter_row_mut(1) {
			*piece = two_leaf;
		}//end looping over row index 1
		for piece in self.board.iter_row_mut(5) {
			*piece = two_leaf;
		}//end looping over row index 5

		// overwrite col index 1 and 5 with two leaf
		for piece in self.board.iter_col_mut(1) {
			*piece = two_leaf;
		}//end looping over col index 1
		for piece in self.board.iter_col_mut(5) {
			*piece = two_leaf;
		}//end looping over col index 5

		// overwrite row index 0 and 6 with one leaf
		for piece in self.board.iter_row_mut(0) {
			*piece = one_leaf;
		}//end looping over row index 0
		for piece in self.board.iter_row_mut(6) {
			*piece = one_leaf;
		}//end looping over row index 6

		// overwrite col index 0 and 6 with one leaf
		for piece in self.board.iter_col_mut(0) {
			*piece = one_leaf;
		}//end looping over col index 0
		for piece in self.board.iter_col_mut(6) {
			*piece = one_leaf;
		}//end looping over col index 6
	}//end initialize_board(&mut self)

	/// # pass_sun_and_moon(&mut self)
	/// 
	/// Carries out the rotation of the moon and sun, updating the area which is in shadow.  
	/// 
	/// Since this function is just for moving the moon and sun, it shouldn't be called every time a player takes a turn
	pub fn pass_sun_and_moon(&mut self) {
		// TODO: Figure out when to flip self.moon.full_moon
		self.sun = self.sun.next();
		self.moon = self.moon.next();
	}//end pass_sun_and_moon(&mut self)

	/// # sun_grid_starts_directions(direciton, rows, cols)
	/// 
	/// Helper method for board.sun_shaded().  
	/// 
	/// Returns an abomination of a tuple. This function has three things to return, so it just throws them all into a tuple.
	/// 
	/// ## parameters
	/// direction : current direciton of the sun
	/// rows : number of rows in current board
	/// cols : number of cols in current board
	/// 
	/// ## return
	/// Aside from the tuple, return has three things:
	/// row_starts : parallel vector of row indices to start at in sun_shaded algorithm
	/// col_starts : parallel vector of col indices to start at in sun_shaded algorithm
	/// row_col_direction : another tuple. This contains the (row,col) direction. So if row is 1 and col is -1, then you would increase in row index and decrease in column index each iteration.
	fn sun_grid_starts_directions(direction: SunDirection, rows: usize, cols: usize) -> (Vec<usize>, Vec<usize>, (i8, i8)) {
		let row_starts: Vec<usize>;
		let col_starts: Vec<usize>;
		let row_col_direction: (i8, i8);

		// bottom row
		let row_starts_north = fill_new_vec(rows, rows - 1);
		let col_starts_north = (0..cols).collect::<Vec<usize>>();
		// leftmost column
		let row_starts_east = (0..rows).collect::<Vec<usize>>();
		let col_starts_east = fill_new_vec(cols, 0);
		// top row
		let row_starts_south = fill_new_vec(rows, 0);
		let col_starts_south = (0..cols).collect::<Vec<usize>>();
		// rightmost column
		let row_starts_west = (0..rows).collect::<Vec<usize>>();
		let col_starts_west = fill_new_vec(cols, cols - 1);

		match direction {
			SunDirection::North => {
				// set starts to bottom row
				row_starts = row_starts_north;
				col_starts = col_starts_north;
				// set direction to just decrease (go up) in row
				row_col_direction = (-1,0);
			}, SunDirection::Northeast => {
				// set starts to bottom left (combination of north and east)
				row_starts = combine_two_vecs(&row_starts_north, &row_starts_east, true);
				col_starts = combine_two_vecs(&col_starts_north, &col_starts_east, true);
				// set direction to decrease in row, increase in column
				row_col_direction = (-1,1);
			}, SunDirection::East => {
				// set starts to leftmost column
				row_starts = row_starts_east;
				col_starts = col_starts_east;
				// set direction to just increase in column
				row_col_direction = (0,1);
			}, SunDirection::Southeast => {
				// set starts to top left (combination of south and east)
				row_starts = combine_two_vecs(&row_starts_south, &row_starts_east, true);
				col_starts = combine_two_vecs(&col_starts_south, &col_starts_east, true);
				// set direction to increase in both row and column (go down and right)
				row_col_direction = (1,1);
			}, SunDirection::South => {
				// set starts to top row
				row_starts = row_starts_south;
				col_starts = col_starts_south;
				// set direction to just increase in row
				row_col_direction = (1, 0);
			}, SunDirection::Southwest => {
				// set starts to top right (combination of south and west)
				row_starts = combine_two_vecs(&row_starts_south, &row_starts_west, true);
				col_starts = combine_two_vecs(&col_starts_south, &col_starts_west, true);
				// set direction to increase in row, decrease in column (go down and left)
				row_col_direction = (1,-1);
			}, SunDirection::West => {
				// set starts to rightmost column
				row_starts = row_starts_west;
				col_starts = col_starts_west;
				// set direction to just decrease (go left) in column
				row_col_direction = (0,-1);
			}, SunDirection::Northwest => {
				// set starts to bottom right
				row_starts = combine_two_vecs(&row_starts_north, &row_starts_west, true);
				col_starts = combine_two_vecs(&col_starts_north, &col_starts_west, true);
				// set direction to decrease in both row and column
				row_col_direction = (-1,-1); },
		}//end matching sun direction

		return (row_starts, col_starts, row_col_direction);
	}//end sun-grid_starts_directions(direction, rows, cols)

	/// # sun_shaded(&self)
	/// 
	/// Function returns parallel grid of booleans.  
	/// 
	/// Each element in grid of booleans says whether that spot is in shadow from the sun.  
	/// 
	/// So, if true, then it is shaded from the sun, and if false, then it does get sunlight.
	/// 
	/// ## Assumptions
	/// 
	/// This function makes some assumptions about the board. Namely, it assumes that the number of rows and columns are both small enough to be expressed as an i8.
	/// 
	/// ## Return
	/// 
	/// This function returns a grid of booleans parallel to self.board. It should be noted, that if a spot would be in shadow, but that spot holds a tree taller than the shadow, such that the tree should still provide light points, then the returned grid will state that that spot is not in shadow. This only happens for trees though. This is done so that one can easily check using this method whether a tree should receive light points or whether a random spot on the board is in shadow for seed planting or tree upgrading purposes.
	pub fn sun_shaded(&self) -> Grid<bool> {
		// instantiate parallel grid
		let mut is_shaded: Grid<bool> = Grid::new(self.board.rows(), self.board.cols());
		is_shaded.fill(false);

		// set some reference variables TBD by sun position
		let start_and_direction = Board::sun_grid_starts_directions(self.sun.direction, self.board.rows(), self.board.cols());
		let row_starts: Vec<usize> = start_and_direction.0;
		let col_starts: Vec<usize> = start_and_direction.1;
		let row_col_direction: (i8, i8) = start_and_direction.2;
		let row_col_starts = merge_two_vecs(&row_starts, &col_starts);

		// sanity check to prevent infinite loop
		if row_col_direction.0 == 0 && row_col_direction.1 == 0 {
			// this should never happen, but if it does, it will prevent infinite loop
			panic!("Direction for row and column cannot be to not move.");
		}//end if direction indicates no movement at all

		// set up loop and start going through things
		for row_col in row_col_starts {
			// need to loop from start, apply row_col_direction, and then if valid, check for tree in inner loop. Only apply shadow in inner loop for each tree
			let mut cur_row = row_col.0 as i8;
			let mut cur_col = row_col.1 as i8;
			let mut shadow_size_left: Vec<(usize, usize)> = Vec::new();
			loop {
				// get the board element at this row_col
				let this_spot = self.board.get(cur_row as usize, cur_col as usize).unwrap();
				
				// test for any objects which would cast a shadow
				let maybe_this_tree = this_spot.tree;
				let is_great_elder_tree = this_spot.piece_type == PieceType::GreatElderTree;
				let is_moonstone = this_spot.piece_type == PieceType::Moonstone;
				
				// apply shadows of normal trees
				if let Some(tree) = maybe_this_tree {
					// add shadow of tree to our funky little Vec
					shadow_size_left.push((tree.size.size(), tree.size.size()));
				}//end if we have a tree to add a shadow to
				// apply shadows of great elder tree, if detected
				if is_great_elder_tree {
					shadow_size_left.push((4,self.board.rows() + self.board.cols()));
				}//end if we have a great elder tree here
				if is_moonstone {
					shadow_size_left.push((1,1));
				}//end if we have a moonstone

				// run through shadow vec to determine if this tree is in shadow
				for shadow in &mut shadow_size_left {
					// if tree here, check if shadow big enough. Else, set shadowed if not great elder tree
					if (maybe_this_tree.is_some() && shadow.0 >= maybe_this_tree.unwrap().size.size()) || (maybe_this_tree.is_none() && !is_great_elder_tree) {
						// set this spot as shaded
						*is_shaded.get_mut(cur_row as usize, cur_col as usize).unwrap() = true;
						// spot already shaded, but don't break, because we need to decrement the other shadows
					}//end if shadow from big enough object
					// decrement the number of tiles left this shadow covers
					shadow.1 -= 1;
				}//end checking for any valid shadows on this spot

				// check if next change would be in bounds
				let row_too_sml = cur_row == 0 && row_col_direction.0 < 0;
				let row_too_big = cur_row == self.board.rows() as i8 - 1 && row_col_direction.0 > 0;
				let col_too_sml = cur_col == 0 && row_col_direction.1 < 0;
				let col_too_big = cur_col == self.board.cols() as i8 - 1 && row_col_direction.1 > 0;
				// update current row and column for next iteration if in bounds
				if !row_too_sml && !row_too_big && !col_too_sml && !col_too_big {
					cur_row += row_col_direction.0;
					cur_col += row_col_direction.1;
				}//end if we're in bounds for next operation
			}//end looping until we hit an invalid index
		}//end looping over the row_col coord starts

		// return updated grid
		return is_shaded;
	}//end sun_shaded(&self)

	/// # moon_grid_starts_directions(direction, rows, cols)
	/// 
	/// Helper method for board.moon_shaded().
	/// 
	/// Returns an abomination of a tuple. This function has four (kinda 8 tbh) things to return, so it just throws them all into a tuple.
	/// 
	/// ## parameters
	/// moon : the moon we're working with
	/// rows : number of rows in current board
	/// cols : number of cols in current board
	/// 
	/// ## return
	/// Aside from the tuple, return has four things:
	/// coord_1_start : The (row,col) coordinate to start the first coordinate at.
	/// direction_1 : The (row,col) direction to go in when starting from coord_1_start.
	/// coord_2_start : The (row,col) coordinate to start the second coordinate at.
	/// direction_2 : The (row,col) direction to go in when starting from coord_2_start.
	fn moon_grid_starts_directions(moon: &Moon) -> ((usize,usize),(i8,i8),(usize,usize),(i8,i8)) {
		let coord_1_start: (usize,usize) = (moon.row1, moon.col1);
		let coord_2_start: (usize,usize) = (moon.row2, moon.col2);
		let direction_1: (i8,i8);
		let direction_2: (i8,i8);

		// set direction based on moon direction
		match moon.direction {
			MoonDirection::North => {
				// left and up
				direction_1 = (-1,-1);
				// right and up
				direction_2 = (-1,1);
			}, MoonDirection::East => {
				// up and right
				direction_1 = (-1,1);
				// down and right
				direction_2 = (1,1);
			}, MoonDirection::South => {
				// right and down
				direction_1 = (1,1);
				// left and down
				direction_2 = (1,-1);
			}, MoonDirection::West => {
				// down and left
				direction_1 = (1,-1);
				// up and left
				direction_2 = (-1,-1); },
		}//end matching direction

		return (coord_1_start, direction_1, coord_2_start, direction_2);
	}//end moon_grid_starts_directions(direction, rows, cols)

	/// # moon_lit(&self)
	/// 
	/// Function returns parallel grid of booleans.  
	/// 
	/// Each element in grid of booleans says whether that spot is lit by the moon.
	/// 
	/// So, if true, then it is lit by the moon, and if false, then it receives no moonlight.
	pub fn moon_lit(&self) -> Grid<bool> {
		// instantiate parallel grid
		let mut is_lit: Grid<bool> = Grid::new(self.board.rows(), self.board.cols());
		is_lit.fill(false);

		// figure a few things out real quick using a helper method
		let moon_starts_and_directions = Board::moon_grid_starts_directions(&self.moon);
		let coord_1_start = moon_starts_and_directions.0;
		let direction_1 = moon_starts_and_directions.1;
		let coord_2_start = moon_starts_and_directions.2;
		let direction_2 = moon_starts_and_directions.3;
		let mut janky_vec: Vec<((usize,usize),(i8,i8))> = Vec::new();
		janky_vec.push((coord_1_start, direction_1));
		janky_vec.push((coord_2_start, direction_2));
		
		// go through and do stuff for each start and direction
		for start_direction in janky_vec {
			// pull our references out of the iterator var
			let start = start_direction.0;
			let direction = start_direction.1;

			let mut cur_row = start.0;
			let mut cur_col = start.1;
			// let shadow_size_left: Vec<(usize,usize)> = Vec::new();
			loop {
				// get the board element at this row_col
				let this_spot = self.board.get(cur_row, cur_col).unwrap();

				// test for any objeccts which would cast a shadow
				// cast giant shadow for great elder tree
				if this_spot.piece_type == PieceType::GreatElderTree {
					break;
				}//end if everything after this is in shadow
				if this_spot.piece_type == PieceType::Moonstone {
					// Get list of adjacent spot, shine light on them, check for more adjacent in loop
					let mut adjacents_queue_index = 0;
					let mut adjacents_queue: Vec<(usize,usize)> = get_adjacent_coords(cur_row, cur_col, self.board.rows() - 1, self.board.cols() - 1, true);
					while adjacents_queue_index < adjacents_queue.len() {
						// set ref variable
						let coord = adjacents_queue.get(adjacents_queue_index).unwrap().clone();
						// figure out the spot for this position
						let this_this_spot = self.board.get(coord.0, coord.1).unwrap();
						
						// check some stuff for later iterations
						if this_this_spot.piece_type == PieceType::Moonstone {
							// get adjacents and add them if adjacents_queue doesn't contain them
							let these_adjacents = get_adjacent_coords(coord.0, coord.1, self.board.rows() - 1, self.board.cols() - 1, true);
							for this_adjacent in these_adjacents {
								if !adjacents_queue.contains(&this_adjacent) {
									adjacents_queue.push(this_adjacent);
								}//end if adjacents_queue doesn't already contain this_adjacent
							}//end adding to adjacents_queue maybe
						}//end if we need to shine on adjacent spots

						// get reference for whether this spot is lit
						let this_lit_spot = is_lit.get_mut(coord.0, coord.1).unwrap();
						*this_lit_spot = true;

						// update for later iterations
						adjacents_queue_index += 1;
					}//end looping over each coord
				}//end if we need to shine onto adjacent spaces
				else {
					// set this spot as moon-lit
					*is_lit.get_mut(cur_row as usize, cur_col as usize).unwrap() = true;
				}//end else we can make this moon-lit

				// check if next change would be in bounds
				let row_too_sml = cur_row == 0 && direction.0 < 0;
				let row_too_big = cur_row == self.board.rows() - 1 && direction.0 > 0;
				let col_too_sml = cur_col == 0 && direction.1 < 0;
				let col_too_big = cur_col == self.board.cols() - 1 && direction.1 > 0;
				// update current row and column for next iteration if in bounds
				if !row_too_sml && !row_too_big && !col_too_sml && !col_too_big {
					cur_row = (cur_row as i8 + direction.0) as usize;
					cur_col = (cur_col as i8 + direction.1) as usize;
				}//end if we're in bounds for next operation
			}//end looping until we finish getting shadows figured out
		}//end looping over starts and directions

		// return updated grid
		return is_lit;
	}//end moon_shaded(&self)
}//end impl for Board

impl Default for Board {
    fn default() -> Self {
        Self {
			board: Grid::new(7,7),
            sun: Sun::default(),
            moon: Moon::new(7),
		}//end struct construction
    }//end default()
}//end impl Default for Board

/// # get_adjacent_coords(&self, max_row, max_col)
/// 
/// This function generates a list of coordinates that are adjacent to this grouping. The maximum row and column index are required in parameters. 
/// This function will automatically exclude coordinates that are already apart of this grouping or that would be out of bounds.
pub fn get_adjacent_coords(row: usize, col: usize, max_row: usize, max_col: usize, allow_diagonal: bool) -> Vec<(usize,usize)> {
	let mut adjacents = Vec::new();
	// top left
	if row > 0 && col > 0 && allow_diagonal {
		let top_left = (row - 1, col - 1);
		adjacents.push(top_left);
	}//end if this coordinate is in bounds
	// top
	if row > 0 {
		let top = (row - 1,col);
		adjacents.push(top);
	}//end if this coordinate is in bounds
	// top right
	if row > 0 && col < max_col && allow_diagonal {
		let top_right = (row - 1,col + 1);
		adjacents.push(top_right);
	}//end if this coordinate is in bounds
	// mid left
	if col > 0 {
		let mid_left = (row, col - 1);
		adjacents.push(mid_left);
	}//end if this coordinate is in bounds
	// mid right
	if col < max_col {
		let mid_right = (row, col + 1);
		adjacents.push(mid_right);
	}//end if this coordinate is in bounds
	// bottom left
	if row < max_row && col > 0 && allow_diagonal {
		let bot_left = (row + 1, col - 1);
		adjacents.push(bot_left);
	}//end if this coordinate is in bounds
	// bottom
	if row < max_row {
		let bot = (row + 1, col);
		adjacents.push(bot);
	}//end if this coordinate is in bounds
	// bottom right
	if row < max_row && col < max_col && allow_diagonal {
		let bot_right = (row + 1, col + 1);
		adjacents.push(bot_right);
	}//end if this coordinate is in bounds
	return adjacents;
}//end get_adjacent_coords()

/// # fill_new_vec<T>(n:usize,value:T)
/// 
/// fills a new vector with specified capacity with the value specified
/// 
/// ## parameters
/// n : the size of the resulting vector  
/// 
/// value : the value to fill every element of the new vec with.
/// 
/// ## return
/// returns a vector of length n, in which every element is value.
fn fill_new_vec<T: Clone>(n:usize,value:T) -> Vec<T> {
	let mut new_vec: Vec<T> = Vec::new();
	for _ in 0..n {
		new_vec.push(value.clone());
	}//end adding new values n times
	return new_vec;
}//end fill_new_vec<T>(n, value)

/// # combine_two_vecs<T>(vec1, vec2, exclude_dupes)
/// 
/// Combines two vectors into one vector. Can optionally exclude duplicates.
/// 
/// ## params
/// vec1 : the first vec to combine  
/// 
/// vec2 : the second vec to combine  
/// 
/// exclude_dupes : whether or not we should exclude duplicates.  
/// 
/// ## return
/// Returns a new vec of type T with all elements of vec1 and vec2.
fn combine_two_vecs<T: Clone + std::cmp::PartialEq>(vec1:&Vec<T>, vec2:&Vec<T>, exclude_dupes:bool) -> Vec<T> {
	let mut new_vec = vec1.clone();

	for item in vec2 {
		if exclude_dupes {
			if new_vec.contains(item) {
				continue;
			}//end if new_vec already contains item
		}//end if we're excluding duplicates
		new_vec.push(item.clone());
	}//end looping over items from vec2

	return new_vec;
}//end combine_two_vecs<T>(vec1, vec2, exclude_dupes)

/// # merge_two_vec<T>(vec1, vec2)
/// 
/// Interlaces two parallel vectors such that for each index n, new_vec[n] = (vec1[n], vec2[n])  
/// 
/// This function will panic if vec1 and vec2 are not the same length
/// 
/// ## params
/// vec1 : the first vector to merge  
/// 
/// vec2 : the second vector to merge
/// 
/// # return
/// Returns a new vector such that for each index n, new_vec[n] = (vec1[n], vec2[n])
#[allow(dead_code)]
fn merge_two_vecs<T: Clone>(vec1:&Vec<T>, vec2:&Vec<T>) -> Vec<(T, T)> {
	// handle case of non-parallel vecs
	if vec1.len() != vec2.len() {
		panic!("Vecs are not parallel!");
	}//end if vecs not same length

	// get into our little function
	let mut new_vec = Vec::new();

	for i in 0..vec1.len() {
		new_vec.push((vec1.get(i).unwrap().clone(), vec2.get(i).unwrap().clone()));
	}//end looping over each index in both vectors

	return new_vec;
}//end merge_two_vecs<T>(vec1, vec2)

/// # BoardSpot
/// 
/// This struct stores the mechanical information for a single spot on the board.  
/// 
/// This information is held through a combination of enums and options.
#[derive(Debug, PartialEq, Eq, Copy, Clone, Ord, PartialOrd, Hash)]
pub struct BoardSpot {
	/// # piece_type
	/// 
	/// The type of piece that this object represents.  
	/// 
	/// Quite important for handling how this piece operates.
	pub piece_type: PieceType,
	/// # tree
	/// 
	/// The tree present at this spot on the board, if there is one.
	pub tree: Option<Tree>,
	/// # animal
	/// 
	/// The type of forest animal that might be on this spot.  
	/// 
	/// Forest animals cannot move onto the same spot as another animal, a moonstone, or the great elder tree.
	pub animal: Option<Animal>,
	/// # fertility
	/// 
	/// The fertility level of this particular spot.  
	/// 
	/// All locations have a fertility level, even if that fertility won't ever be used.
	pub fertility: Fertility,
	/// # expended
	/// 
	/// Whether or not this spot has been expended this turn.  
	/// 
	/// An spot on the board from which as action has been taken is expended.  
	/// 
	/// If a spot is expended, you can't use it for anything else until the next turn.
	expended: bool,
}//end struct BoardSpot

#[allow(dead_code)]
impl BoardSpot {
	/// # new()
	/// 
	/// Creates a new BoardSpot identical to default() with one exception.  
	/// 
	/// Fertility has no real default, so you must specify it.  
	/// 
	/// If you want to use the default Fertility of OneLeaf, use default().
	pub fn new(fertility: Fertility) -> BoardSpot {
		BoardSpot { 
			piece_type: Default::default(),
			tree: None,
			animal: None,
			fertility,
			expended: false,
		}//end struct construction
	}//end new(fertility/)

	/// # is_expended(self)
	/// 
	/// Returns whether or not this spot has been expended.  
	/// 
	/// An spot on the board from which as action has been taken is expended.  
	/// 
	/// If a spot is expended, you can't use it for anything else until the next turn.  
	/// 
	pub fn is_expended(&self) -> bool {
		return self.expended;
	}//end is_expended(&self)

	/// # expend(&mut self)
	/// 
	/// Will expend this spot until the next turn.
	pub fn expend(&mut self) {
		self.expended = true;
	}//end expend(self)
}//end impl for BoardSpot

impl Default for BoardSpot {
    fn default() -> Self {
        Self {
			piece_type: PieceType::Empty,
			tree: None,
			animal: None,
			fertility: Fertility::OneLeaf,
			expended: false,
		}//end struct construction
    }//end default()
}//end impl Default for BoardSpot

#[derive(Debug, PartialEq, Eq, Copy, Clone, Ord, PartialOrd, Hash)]
/// # Tree
/// 
/// Represents a single tree on the board.
pub struct Tree {
	/// The color of this particular tree. The color denotes the owner of the tree.
	color:(u8,u8,u8),
	/// The size of the tree.
	size:TreeSize,
}//end struct Tree

impl Default for Tree {
    fn default() -> Self {
        Self {
			color: (0,0,0),
			size: TreeSize::Seed,
		}//end struct construction
    }//end default()
}//end impl Default for Tree
