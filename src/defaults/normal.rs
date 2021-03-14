use crate::{Board, Piece, Mode, Player};
use crate::defaults::types::PieceType;
use glucose::linear::vec::Point;

pub struct Default8x8 {
    board: Board<2>,
    pieces: Vec<Piece<2>>,
    next_move: Option<(i32, Point<i32, 2>)>
}

impl Default8x8 {
    fn default_setup(&mut self) {
        let mut id = 0;

        for pos in 0..8 {
            self.pieces[id] = Piece::new(0, id as i32, PieceType::Pawn.into(), Point::from([1, pos]));
            id += 1;
        }

        self.pieces[id] = Piece::new(0, id as i32, PieceType::Rook.into(), Point::from([0, 0]));
        id += 1;

        self.pieces[id] = Piece::new(0, id as i32, PieceType::Rook.into(), Point::from([0, 7]));
        id += 1;

        self.pieces[id] = Piece::new(0, id as i32, PieceType::Knight.into(), Point::from([0, 1]));
        id += 1;
        self.pieces[id] = Piece::new(0, id as i32, PieceType::Knight.into(), Point::from([0, 6]));
        id += 1;

        self.pieces[id] = Piece::new(0, id as i32, PieceType::Bishop.into(), Point::from([0, 2]));
        id += 1;
        self.pieces[id] = Piece::new(0, id as i32, PieceType::Bishop.into(), Point::from([0, 5]));
        id += 1;

        self.pieces[id] = Piece::new(0, id as i32, PieceType::Queen.into(), Point::from([0, 3]));
        id += 1;
        self.pieces[id] = Piece::new(0, id as i32, PieceType::King.into(), Point::from([0, 4]));
        id += 1;

        for pos in 0..8 {
            self.pieces[id] = Piece::new(1, id as i32, PieceType::Pawn.into(), Point::from([6, pos]));
            id += 1;
        }

        self.pieces[id] = Piece::new(1, id as i32, PieceType::Rook.into(), Point::from([7, 0]));
        id += 1;

        self.pieces[id] = Piece::new(1, id as i32, PieceType::Rook.into(), Point::from([7, 7]));
        id += 1;

        self.pieces[id] = Piece::new(1, id as i32, PieceType::Knight.into(), Point::from([7, 1]));
        id += 1;
        self.pieces[id] = Piece::new(1, id as i32, PieceType::Knight.into(), Point::from([7, 6]));
        id += 1;

        self.pieces[id] = Piece::new(1, id as i32, PieceType::Bishop.into(), Point::from([7, 2]));
        id += 1;
        self.pieces[id] = Piece::new(1, id as i32, PieceType::Bishop.into(), Point::from([7, 5]));
        id += 1;

        self.pieces[id] = Piece::new(1, id as i32, PieceType::Queen.into(), Point::from([7, 3]));
        id += 1;
        self.pieces[id] = Piece::new(1, id as i32, PieceType::King.into(), Point::from([7, 4]));
    }

    fn handle_input(&mut self, input: String, current_player: i32) -> Result<Option<String>, String> {
        let (start, end) = Self::convert_input_to_move(input);
        let (player_id, piece_id, piece_type) = self.find_piece(start).unwrap();

        Ok(None)
    }

    fn find_piece(&self, pos: Point<i32, 2>) -> Option<(i32, i32, i32)> {
        let maybe_piece = self.pieces.iter().find(|piece| piece.pos == pos);
        match maybe_piece {
            None => None,
            Some(piece) => Some((piece.player, piece.id, piece.ty))
        }
    }

    fn convert_input_to_move(input: String) -> (Point<i32, 2>, Point<i32, 2>) {
        let split: Vec<String> = input.split(" -> ").into_iter().map(|pos| pos.to_string()).collect();
        let start = Point::from(split[0].clone());
        let end = Point::from(split[1].clone());
        (start, end)
    }
}

impl Mode for Default8x8 {
    const PLAYERS: usize = 2;
    const STARTING_PLAYER: i32 = 0;
    const DIMENSIONS: usize = 2;

    fn new() -> Self {
        let mut default = Self {
            board: Board::new([8, 8]),
            pieces: vec![Piece::default(); 32],
            next_move: None,
        };

        default.default_setup();

        default
    }

    fn create_player(&self) -> Vec<Player> {
        let p0 = Player::new(self.pieces[0..15].iter().map(|piece| piece.id).collect());
        let p1 = Player::new(self.pieces[0..15].iter().map(|piece| piece.id).collect());
        vec![p0, p1]
    }

    fn next_move(&mut self, input: String, player: i32) -> Result<Option<String>, String> {
        unimplemented!()
    }
}

