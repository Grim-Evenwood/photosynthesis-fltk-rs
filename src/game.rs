use grid::Grid;
use self::enums::{PieceType, Animal, TreeSize, Fertility};
mod enums;

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
}//end struct BoardPiece

#[allow(dead_code)]
impl BoardPiece {
	pub fn new(fertility: Fertility) -> BoardPiece {
		BoardPiece { 
			piece_type: Default::default(), 
			animal: Default::default(), 
			tree_size: Default::default(), 
			fertility, 
		}//end struct construction
	}//end new(fertility/)
}//end impl for BoardPiece

impl Default for BoardPiece {
    fn default() -> Self {
        Self {
			piece_type: Default::default(),
			animal: Default::default(),
			tree_size: Default::default(),
			fertility: Fertility::OneLeaf,
		}//end struct construction
    }//end default()
}//end impl Default for BoardPiece