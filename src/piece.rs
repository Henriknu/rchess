use std::fmt::Display;

#[derive(Debug, Clone, Copy)]
pub struct Piece {
    ty: PieceType,
    color: Color,
}

impl Piece {
    pub fn new(ty: PieceType, color: Color) -> Self {
        Self { ty, color }
    }
    pub fn pawn(color: Color) -> Self {
        Self {
            ty: PieceType::Pawn,
            color,
        }
    }
    pub fn rook(color: Color) -> Self {
        Self {
            ty: PieceType::Rook,
            color,
        }
    }
    pub fn bishop(color: Color) -> Self {
        Self {
            ty: PieceType::Bishop,
            color,
        }
    }
    pub fn knight(color: Color) -> Self {
        Self {
            ty: PieceType::Knight,
            color,
        }
    }
    pub fn queen(color: Color) -> Self {
        Self {
            ty: PieceType::Queen,
            color,
        }
    }
    pub fn king(color: Color) -> Self {
        Self {
            ty: PieceType::King,
            color,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Color {
    Black,
    White,
}

#[derive(Debug, Clone, Copy)]
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
        let c = match self.ty {
            PieceType::Pawn => '*',
            PieceType::Knight => 'N',
            PieceType::Bishop => 'B',
            PieceType::Rook => 'R',
            PieceType::Queen => 'Q',
            PieceType::King => 'K',
        };

        write!(f, "{}", c)
    }
}
