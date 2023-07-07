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
/// It stores position in a Grid of BoardPiece objects,
/// and each BoardPiece object stores mechanic information about that position.
/// 
/// It should be noted that several methods will assume that the board is a grid of 7x7, and they will not work without that assumption.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Board {
	/// the grid that represents the game board
	pub board:Grid<BoardPiece>,
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
		let one_leaf = BoardPiece::new(Fertility::OneLeaf);
		let two_leaf = BoardPiece::new(Fertility::TwoLeaf);
		let three_leaf = BoardPiece::new(Fertility::ThreeLeaf);
		let four_leaf = BoardPiece::new(Fertility::FourLeaf);

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
		todo!();
	}//end pass_sun_and_moon(&mut self)

	/// # sun_shaded(&self)
	/// 
	/// Function returns parallel grid of booleans.  
	/// 
	/// Each element in grid of booleans says whether that spot is in shadow from the sun.  
	/// 
	/// So, if true, then it is shaded from the sun, and if false, then it does get sunlight.
	pub fn sun_shaded(&self) -> Grid<bool> {
		// instantiate parallel grid
		let mut is_shaded: Grid<bool> = Grid::new(self.board.rows(), self.board.cols());
		is_shaded.fill(false);

		// TODO: Figure out whether each position is shaded


		// return updated grid
		return is_shaded;
	}//end sun_shaded(&self)

	/// # moon_shaded(&self)
	/// 
	/// Function returns parallel grid of booleans.  
	/// 
	/// Each element in grid of booleans says whether that spot is in shadow from the moon.
	/// 
	/// So, if true, then it is shaded from the moon, and if false, then it does get moonlight.
	pub fn moon_shaded(&self) -> Grid<bool> {
		// instantiate parallel grid
		let mut is_shaded: Grid<bool> = Grid::new(self.board.rows(), self.board.cols());
		is_shaded.fill(false);

		// TODO: Figure out whether each position is shaded

		// return updated grid
		return is_shaded;
	}//end moon_shaded(&self)
}//end impl for Board

impl Default for Board {
    fn default() -> Self {
        Self {
			board: Grid::new(7,7),
		}//end struct construction
    }//end default()
}//end impl Default for Board

/// # BoardPiece
/// 
/// This struct stores the mechanical information for a single spot on the board.  
/// 
/// This information is held through a combination of enums and options.
#[derive(Debug, PartialEq, Eq, Copy, Clone, Ord, PartialOrd, Hash)]
pub struct BoardPiece {
	/// # piece_type
	/// 
	/// The type of piece that this object represents.
	/// Quite important for handling how this piece operates.
	pub piece_type: PieceType,
	/// # animal
	/// 
	/// The type of forest animal that might be on this spot.
	/// Forest animals cannot move onto the same spot as another animal, a moonstone, or the great elder tree.
	pub animal: Option<Animal>,
	/// # tree_size
	/// 
	/// The size of tree which might be present on this space.
	/// Not relevant unless piece_type is PieceType::Tree.
	pub tree_size: Option<TreeSize>,
	/// # fertility
	/// 
	/// The fertility level of this particular spot.
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
}//end struct BoardPiece

#[allow(dead_code)]
impl BoardPiece {
	/// # new()
	/// 
	/// Creates a new BoardPiece identical to default() with one exception.  
	/// 
	/// Fertility has no real default, so you must specify it.  
	/// 
	/// If you want to use the default Fertility of OneLeaf, use default().
	pub fn new(fertility: Fertility) -> BoardPiece {
		BoardPiece { 
			piece_type: Default::default(), 
			animal: Default::default(), 
			tree_size: Default::default(), 
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
}//end impl for BoardPiece

impl Default for BoardPiece {
    fn default() -> Self {
        Self {
			piece_type: Default::default(),
			animal: Default::default(),
			tree_size: Default::default(),
			fertility: Fertility::OneLeaf,
			expended: false,
		}//end struct construction
    }//end default()
}//end impl Default for BoardPiece