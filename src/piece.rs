use colored::Colorize;
use rand::{self, random};
use std::fmt::Display;

use crate::{
    board::Board,
    ply::{pawn_plys, Ply},
};

#[derive(Debug, Clone, Copy)]
pub struct Piece {
    pub id: u64,
    ty: PieceType,
    color: Color,
}

impl Piece {
    fn new(ty: PieceType, color: Color) -> Self {
        Self {
            id: random(),
            ty,
            color,
        }
    }
    pub fn pawn(color: Color) -> Self {
        Self::new(PieceType::Pawn, color)
    }
    pub fn rook(color: Color) -> Self {
        Self::new(PieceType::Rook, color)
    }
    pub fn bishop(color: Color) -> Self {
        Self::new(PieceType::Bishop, color)
    }
    pub fn knight(color: Color) -> Self {
        Self::new(PieceType::Knight, color)
    }
    pub fn queen(color: Color) -> Self {
        Self::new(PieceType::Queen, color)
    }
    pub fn king(color: Color) -> Self {
        Self::new(PieceType::King, color)
    }

    pub fn color(&self) -> Color {
        self.color
    }

    pub fn ty(&self) -> PieceType {
        self.ty
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Color {
    Black,
    White,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

impl Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self.ty {
            PieceType::Pawn => "*",
            PieceType::Knight => "N",
            PieceType::Bishop => "B",
            PieceType::Rook => "R",
            PieceType::Queen => "Q",
            PieceType::King => "K",
        };

        let colored_text = match self.color {
            Color::Black => s.blue(),
            Color::White => s.white(),
        };

        write!(f, "{}", colored_text)
    }
}
