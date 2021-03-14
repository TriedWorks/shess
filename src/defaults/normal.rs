use crate::{Board, Piece, Mode, Player, Move, RenderMove2D};
use crate::defaults::types::PieceType;
use glucose::linear::vec::Point;
use fructose::algebra::linear::vector::NormedSpace;
use crate::cache::MoveCache;

pub struct Default8x8 {
    board: Board<2>,
    pieces: Vec<Piece<2>>,
    cache: MoveCache<2>,
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

        let (player_id, piece_id, piece_type) = match self.find_piece(start) {
            None => {
                return Err(String::from(format!("Square: ({}, {}) is empty", start[0], end[1])))
            }
            Some((pli, pid, pit)) => (pli, pid, pit)
        };

        if player_id != current_player {
            return Err(String::from("Not your figure"))
        }

        if !self.check_bounds(end) {
            return Err(String::from("Position is out of Bounds"))
        }

        if !Self::check_valid_move(piece_type.into(), start, end) {
            return Err(String::from("Invalid Move"))
        }

        if !self.check_unoccupied(end) {
            // TODO Add castling here!
            return Err(String::from("Already Occupied"))
        }

        self.next_move = Some((piece_id, end));

        Ok(None)
    }

    fn find_piece(&self, pos: Point<i32, 2>) -> Option<(i32, i32, i32)> {
        let maybe_piece = self.pieces.iter().find(|piece| piece.pos == pos);
        match maybe_piece {
            None => None,
            Some(piece) => Some((piece.player, piece.id, piece.ty))
        }
    }

    fn check_bounds(&self, point: Point<i32, 2>) -> bool {
        point[0] >= self.board.start[0] && point[1] >= self.board.start[1]
        && point[0] < self.board.size[0] && point[1] < self.board.size[1]
    }

    fn check_unoccupied(&self, point: Point<i32, 2>) -> bool {
        self.pieces.iter().find(|piece| piece.pos == point).is_some()
    }

    fn check_valid_move(pt: PieceType, start: Point<i32, 2>, end: Point<i32, 2>) -> bool {
        let vec = start - end;
        let d_width = (start[0] - end[0]).abs();
        let d_height = (start[1] - end[1]).abs();
        let mag = Point::from([vec[0] as f32, vec[1] as f32]).norm() as i32;

        match pt {
            PieceType::King => {
                mag == 1
            }
            PieceType::Queen => {
                d_width == d_height
                    || (d_width != 0 && d_height == 0)
                    || (d_width == 0 && d_height != 0)
            }
            PieceType::Rook => {
                d_width != 0 && (d_height == 0)
                    || (d_width == 0) && (d_height != 0)
            }
            PieceType::Bishop => {
                d_width == d_height
            }
            PieceType::Knight => {
                d_width == 2 && d_height == 1
                || d_height == 2 || d_width == 1
            }

            PieceType::Pawn => {
                d_height == 1
            }
        }
    }

    fn move_piece(&mut self, player: i32) -> Move {
        match self.next_move {
            None => { panic!() }
            Some(next) => {
                let piece = self.pieces.iter_mut().find(|piece| piece.id == next.0).unwrap();

                let mo = Move {
                    player_id: player,
                    piece_id: next.0,
                    from: Some(piece.pos),
                    to: next.1
                };

                piece.pos = next.1;
                self.next_move = None;

                mo
            }
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
            board: Board::new(Point::from([0, 0]), [8, 8]),
            pieces: vec![Piece::default(); 32],
            cache: MoveCache::new_with_capacity(64),
            next_move: None,
        };

        default.default_setup();

        default
    }

    fn create_player(&self) -> Vec<Player> {
        let p0 = Player::new(0);
        let p1 = Player::new(1);
        vec![p0, p1]
    }

    fn next_move(&mut self, input: String, player: i32) -> Result<Option<String>, String> {
        match self.handle_input(input, player) {
            Ok(result) => {
                Ok(result)
            }
            Err(err) => {
                Err(err)
            }
        }
    }

    fn execute_move(&mut self, player: i32) {
        let mo = self.move_piece(player);
        self.cache.cache.push(mo);
    }

    fn board(&self) -> (Vec<RenderMove2D>, usize) {
        let moves = Vec::with_capacity(64);
    }
}
