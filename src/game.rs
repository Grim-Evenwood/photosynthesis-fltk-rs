use grid::Grid;
use self::enums::{PieceType, Animal, TreeSize, Fertility, SunDirection, MoonDirection, MovingLightDirection};
mod enums;

#[derive(Debug, PartialEq, Eq, Copy, Clone, Ord, PartialOrd, Hash)]
pub struct Sun {
	direction: SunDirection,
}//end struct Sun

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
/// The moon sort of sits in between two spots on a grid, and it shines diagonally in two direction.
/// For this reason, the moon holds two coordinates, and sits between them.
/// It also has a handy function to calculate the next position.
#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq, Copy, Clone, Ord, PartialOrd, Hash)]
pub struct Moon {
	pub direction: MoonDirection,
	pub row1: usize,
	pub col1: usize,
	pub row2: usize,
	pub col2: usize,
	grid_side_length: usize,
}//end struct Moon

#[allow(dead_code)]
impl Moon {
	/// # new(grid_side_length)
	/// 
	/// This function creates a new moon object.
	/// Moon objects are used with square grids, and you must provide the length of each side of that square grid, in terms of rows/cols.
	/// The moon will start out pointing south, near the upper top right corner of the board.
	/// This function will fail if the side length provided is less than 2.
	pub fn new(grid_side_length: usize) -> Moon {
		Moon {
			direction: MoonDirection::South,
			row1: 0,
			row2: 0,
			col1: grid_side_length - 2,
			col2: grid_side_length - 1,
			grid_side_length,
		}//end struct construction.
	}//end new(grid_side_length)

	/// # next(&self)
	/// 
	/// This function calculates the next position of this moon object, based on the given grid side length.
	pub fn next(&self) -> Moon {
		/// this counter will be used to help us move things.
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
			grid_side_length: self.grid_side_length,
		}//end struct construction
	}//end next(&self)
}//end impl for Moon

/// # Board struct
/// 
/// This struct stores the state for the whole game board.
/// It stores position in a Grid of BoardPiece objects,
/// and each BoardPiece object stores mechanic information about that position.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Board {
	/// the grid that represents the game board
	pub board:Grid<BoardPiece>,
}//end struct Board

#[allow(dead_code)]
impl Board {
	pub fn pass_turn(&mut self) {
		todo!();
	}//end pass_turn(&mut self)
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
	/// An spot on the board from which as action has been taken is expended.
	/// If a spot is expended, you can't use it for anything else until the next turn.
	expended: bool,
}//end struct BoardPiece

#[allow(dead_code)]
impl BoardPiece {
	/// # new()
	/// 
	/// Creates a new BoardPiece identical to default() with one exception.
	/// Fertility has no real default, so you must specify it.
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
	/// An spot on the board from which as action has been taken is expended.
	/// If a spot is expended, you can't use it for anything else until the next turn.
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