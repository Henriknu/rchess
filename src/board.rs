use crate::piece::{Color, Piece};

type PositionMatrix = [PositionRow; 8];
type PositionRow = [Position; 8];

pub struct Board {
    inner: PositionMatrix,
}

impl Board {
    pub fn new() -> Self {
        let mut inner = [[Default::default(); 8]; 8];

        Self::init_positions(&mut inner);

        Self::populate_piece_row(&mut inner[0], Color::Black);
        Self::populate_pawns_row(&mut inner[1], Color::Black);

        Self::populate_pawns_row(&mut inner[6], Color::White);
        Self::populate_piece_row(&mut inner[7], Color::White);

        Self { inner }
    }

    pub fn print(&self) {
        let border = "-".repeat(17);

        println!(" {}", border);
        for row in self.inner {
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

    fn init_positions(inner: &mut PositionMatrix) {
        for (idx_r, row) in inner.iter_mut().enumerate() {
            for (idx_c, cell) in row.iter_mut().enumerate() {
                cell.row = idx_r;
                cell.col = idx_c;
            }
        }
    }

    fn populate_piece_row(row: &mut PositionRow, color: Color) {
        row[0].piece = Some(Piece::rook(color));
        row[7].piece = Some(Piece::rook(color));
        row[1].piece = Some(Piece::knight(color));
        row[6].piece = Some(Piece::knight(color));
        row[2].piece = Some(Piece::bishop(color));
        row[5].piece = Some(Piece::bishop(color));
        row[3].piece = Some(Piece::king(color));
        row[4].piece = Some(Piece::queen(color));
    }

    fn populate_pawns_row(row: &mut PositionRow, color: Color) {
        for place in row {
            place.piece = Some(Piece::pawn(color));
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Position {
    col: usize,
    row: usize,
    piece: Option<Piece>,
}

impl Position {
    pub fn new(col: usize, row: usize, piece: Option<Piece>) -> Self {
        Self { col, row, piece }
    }
}
