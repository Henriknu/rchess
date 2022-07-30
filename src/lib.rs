use std::fmt::Display;

pub struct Board {
    inner: [[Option<Piece>; 8]; 8],
}

impl Board {
    pub fn new() -> Self {
        let mut inner = [[None; 8]; 8];

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
                if let Some(piece) = cell {
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

    fn populate_piece_row(row: &mut [Option<Piece>; 8], color: Color) {
        row[0] = Some(Piece::rook(color));
        row[7] = Some(Piece::rook(color));
        row[1] = Some(Piece::knight(color));
        row[6] = Some(Piece::knight(color));
        row[2] = Some(Piece::bishop(color));
        row[5] = Some(Piece::bishop(color));
        row[3] = Some(Piece::king(color));
        row[4] = Some(Piece::queen(color));
    }

    fn populate_pawns_row(row: &mut [Option<Piece>; 8], color: Color) {
        for place in row {
            *place = Some(Piece::pawn(color));
        }
    }
}

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
