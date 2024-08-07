use crate::{
    board::Board,
    piece::{Piece, PieceType},
    ply::{king_plys, pawn_plys, Ply},
};

#[derive(Debug, Clone, Copy, Default)]
pub struct Square {
    col: usize,
    row: usize,
    pub piece: Option<Piece>,
}

impl Square {
    pub fn new(col: usize, row: usize, piece: Option<Piece>) -> Self {
        Self { col, row, piece }
    }

    pub fn col(&self) -> usize {
        self.col + 1
    }

    pub fn row(&self) -> usize {
        self.row + 1
    }

    pub fn is_unoccupied(&self) -> bool {
        self.piece.is_none()
    }

    pub fn is_occupied_by_opponent(&self, other: &Square) -> bool {
        if other.piece.is_none() {
            return false;
        }

        if self.piece.is_none() {
            return false;
        }

        self.piece.unwrap().color() != other.piece.unwrap().color()
    }

    pub fn valid_plys(&self, board: &Board) -> Vec<Ply> {
        if let Some(piece) = self.piece {
            match piece.ty() {
                PieceType::Pawn => return pawn_plys(board, &self),
                PieceType::Knight => todo!(),
                PieceType::Bishop => todo!(),
                PieceType::Rook => todo!(),
                PieceType::Queen => todo!(),
                PieceType::King => return king_plys(board, &self),
            }
        }

        vec![]
    }
}
