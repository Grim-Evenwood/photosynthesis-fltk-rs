use grid::Grid;

use crate::enums::PieceType;
mod enums;

pub struct Board {
	/// the grid that represents the game board
	pub board:Grid<String>,
}//end struct Board

pub struct BoardPiece {
	pub piece_type: PieceType,
}//end struct BoardPiece
