#[derive(Debug, Copy, Clone)]
pub enum PieceType {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}

impl From<PieceType> for i32 {
    fn from(piece: PieceType) -> Self {
        match piece {
            PieceType::King => 5,
            PieceType::Queen => 4,
            PieceType::Rook => 3,
            PieceType::Bishop => 2,
            PieceType::Knight => 1,
            PieceType::Pawn => 0,
        }
    }
}

impl From<i32> for PieceType {
    fn from(num: i32) -> Self {
        match num {
            5 => PieceType::King,
            4 => PieceType::Queen,
            3 => PieceType::Rook,
            2 => PieceType::Bishop,
            1 => PieceType::Knight,
            0 => PieceType::Pawn,
            _ => panic!("Invalid number!"),
        }
    }
}
