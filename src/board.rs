use crate::{
    piece::{Color, Piece},
    ply::Ply,
    square::Square,
};

type Matrix = [Row; 8];
type Row = [Square; 8];

#[derive(Debug, Clone)]
pub struct Board {
    inner: Matrix,
}

impl Board {
    pub fn new() -> Self {
        let mut inner = Self::construct_matrix();

        Self::populate_piece_row(&mut inner[0], Color::White);
        Self::populate_pawns_row(&mut inner[1], Color::White);

        Self::populate_pawns_row(&mut inner[6], Color::Black);
        Self::populate_piece_row(&mut inner[7], Color::Black);

        Self { inner }
    }

    pub fn square(&self, row: usize, col: usize) -> &Square {
        &self.inner[row - 1][col - 1]
    }

    pub fn piece(&self, piece_id: u64) -> Option<&Square> {
        for row in &self.inner {
            for square in row {
                if let Some(piece) = square.piece {
                    if piece.id == piece_id {
                        return Some(square);
                    }
                }
            }
        }
        None
    }

    pub fn make_ply(&mut self, ply: Ply) {
        let from_piece = ply
            .from()
            .piece
            .expect("Invalid ply, piece not in from square.");

        self.inner[ply.from().row() - 1][ply.from().col() - 1].piece = None;
        self.inner[ply.to().row() - 1][ply.to().col() - 1].piece = Some(from_piece);
    }

    pub fn get_relative_square(&self, square: &Square, row: isize, col: isize) -> Option<&Square> {
        let new_row = square.row() as isize + row;
        let new_col = square.col() as isize + col;

        if !Self::within_bounds(new_row) || !Self::within_bounds(new_col) {
            return None;
        }

        Some(&self.inner[new_row as usize - 1][new_col as usize - 1])
    }

    pub fn print(&self) {
        let border = "-".repeat(17);

        println!(" {}", border);
        for row in self.inner.iter().rev() {
            print!("| ");
            for cell in row {
                if let Some(piece) = cell.piece {
                    print!("{}", piece);
                } else {
                    print!(" ");
                }
                print!(" ");
            }
            print!("|");
            println!()
        }
        println!(" {}", border);
    }

    fn construct_matrix() -> Matrix {
        let mut matrix = [[Default::default(); 8]; 8];

        for (idx_r, row) in matrix.iter_mut().enumerate() {
            for (idx_c, square) in row.iter_mut().enumerate() {
                *square = Square::new(idx_c, idx_r, None);
            }
        }

        matrix
    }

    fn populate_piece_row(row: &mut Row, color: Color) {
        row[0].piece = Some(Piece::rook(color));
        row[7].piece = Some(Piece::rook(color));
        row[1].piece = Some(Piece::knight(color));
        row[6].piece = Some(Piece::knight(color));
        row[2].piece = Some(Piece::bishop(color));
        row[5].piece = Some(Piece::bishop(color));
        row[3].piece = Some(Piece::queen(color));
        row[4].piece = Some(Piece::king(color));
    }

    fn populate_pawns_row(row: &mut Row, color: Color) {
        for place in row {
            place.piece = Some(Piece::pawn(color));
        }
    }

    #[inline]
    fn within_bounds(value: isize) -> bool {
        value < 9 && value >= 1
    }
}
