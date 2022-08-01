use crate::{
    board::Board,
    piece::{Color, Piece, PieceType},
    square::Square,
};

#[derive(Debug, Clone, Copy)]
pub struct Ply {
    from: Square,
    to: Square,
}

impl Ply {
    pub fn new(from: Square, to: Square) -> Self {
        Self { from, to }
    }

    pub fn from(&self) -> Square {
        self.from
    }

    pub fn to(&self) -> Square {
        self.to
    }
}

pub fn pawn_plys(board: &Board, square: &Square) -> Vec<Ply> {
    let mut plys = vec![];

    let piece = square.piece.expect("Ply must include pawn piece");

    assert_eq!(piece.ty(), PieceType::Pawn);

    // move forward, two squares if starting position, else one square

    let forward_row = if piece.color() == Color::White { 1 } else { -1 };

    if let Some(forward_square) = board.get_relative_square(square, forward_row, 0) {
        if forward_square.is_unoccupied() {
            plys.push(Ply::new(*square, *forward_square))
        }
    }

    if is_pawn_start_square(&square, &piece) {
        if let Some(forward_forward_square) = board.get_relative_square(square, forward_row * 2, 0)
        {
            if forward_forward_square.is_unoccupied() {
                plys.push(Ply::new(*square, *forward_forward_square))
            }
        }
    }

    // Capture diagonally

    if let Some(left_diagonal_square) = board.get_relative_square(square, forward_row, -1) {
        if let Some(other_piece) = left_diagonal_square.piece {
            if piece.color() != other_piece.color() {
                plys.push(Ply::new(*square, *left_diagonal_square))
            }
        }
    }

    if let Some(right_diagonal_square) = board.get_relative_square(square, forward_row, 1) {
        if let Some(other_piece) = right_diagonal_square.piece {
            if piece.color() != other_piece.color() {
                plys.push(Ply::new(*square, *right_diagonal_square))
            }
        }
    }

    // TODO: Enpasseant

    // TODO: Cannot ply if king is in check

    plys
}

pub fn knight_plys(board: &Board, piece: &Piece) -> Vec<Ply> {
    todo!()
}

pub fn bishop_plys(board: &Board, piece: &Piece) -> Vec<Ply> {
    todo!()
}

pub fn rook_plys(board: &Board, piece: &Piece) -> Vec<Ply> {
    todo!()
}

pub fn queen_plys(board: &Board, piece: &Piece) -> Vec<Ply> {
    todo!()
}

pub fn king_plys(board: &Board, piece: &Piece) -> Vec<Ply> {
    todo!()
}

pub fn is_pawn_start_square(square: &Square, piece: &Piece) -> bool {
    square.row() == 2 && piece.color() == Color::White
        || square.row() == 7 && piece.color() == Color::Black
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn pawn() {
        let mut board = Board::new();

        // e4

        let e2 = board.square(2, 5);

        let plys = e2.valid_plys(&board);

        assert_eq!(plys.len(), 2);

        board.make_ply(plys[1]);

        // d5

        let d7 = board.square(7, 4);

        let plys = d7.valid_plys(&board);

        board.make_ply(plys[1]);

        //exd5

        let e4 = board.square(4, 5);

        let plys = e4.valid_plys(&board);

        assert_eq!(plys.len(), 2);
    }
}
