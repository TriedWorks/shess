use glucose::linear::vec::Point;

pub mod defaults;
#[cfg(feature = "discord")]
pub mod discord;
#[cfg(feature = "terminal")]
pub mod terminal;
pub mod cache;

#[derive(Debug, Copy, Clone)]
pub struct Move<const N: usize> {
    player_id: i32,
    piece_id: i32,
    from: Option<Point<i32, { N }>>,
    to: Point<i32, { N }>
}

#[derive(Debug, Copy, Clone)]
pub struct RenderMove2D {
    player_id: i32,
    piece_id: i32,
    pos: Point<i32, 2>
}

pub struct Player {
    id: i32
}

impl Player {
    pub const fn new(id: i32) -> Self {
        Self {
            id
        }
    }
}

#[derive(Debug, Default, Copy, Clone)]
pub struct Piece<const N: usize> {
    player: i32,
    id: i32,
    sub_id: Option<i32>,
    ty: i32,
    pos: Point<i32, { N }>,
}

impl<const N: usize> Piece<{ N }> {
    pub fn new(player: i32, id: i32, ty: i32, pos: Point<i32, { N }>) -> Self {
        Self { player, id, sub_id: None, ty, pos }
    }

    pub fn new_with_sub(player: i32, id: i32, sub_id: i32, ty: i32, pos: Point<i32, { N }>) -> Self {
        Self { player, id, sub_id: Some(sub_id), ty, pos }
    }
}

pub struct Board<const DIMS: usize> {
    start: Point<i32, { DIMS }>,
    size: [i32; DIMS],
}

impl<const DIMS: usize> Board<{ DIMS }> {
    pub fn new(start: Point<i32, { DIMS }>, size: [i32; DIMS]) -> Self {
        Self {
            start,
            size,
        }
    }
}


pub trait Mode {
    const PLAYERS: usize;
    const STARTING_PLAYER: i32;
    const DIMENSIONS: usize;

    fn new() -> Self;

    fn create_player(&self) -> Vec<Player>;

    fn next_move(&mut self, input: String, player: i32) -> Result<Option<String>, String>;

    fn execute_move(&mut self, player: i32);

    fn board(&self) -> (Vec<RenderMove2D>, usize);
}

pub trait Backend {
    fn new() -> Self;

    fn receive(&self) -> Result<Option<String>, String>;

    fn send(&mut self, msg: String) -> Result<Option<String>, String>;
}

pub struct Game<M: Mode, B: Backend> {
    mode: M,
    backend: B,
    players: Vec<Player>,
    current_player: i32,
}

impl<M: Mode, B: Backend> Game<M, B> {
    pub fn new() -> Self {
        let mode = M::new();
        let backend = B::new();

        let players = mode.create_player();

        Self {
            mode,
            backend,
            players,
            current_player: M::STARTING_PLAYER,
        }
    }

    pub fn next_move(&mut self) {
        let mut input = self.backend.receive();
        while input.is_err() {
            match self.backend.send(String::from("Wrong Input")) {
                Ok(_) => {}
                Err(err) => {panic!(format!("{}", err))}
            };
            input = self.backend.receive();
        }
        match self.mode.next_move(input.unwrap().unwrap(), self.current_player) {
            Ok(_) => {}
            Err(err) => {
                match self.backend.send(String::from(format!("Error in Move: {}", err))) {
                    Ok(_) => {}
                    Err(err) => {panic!(format!("{}", err))}
                };
                self.next_move()
            }
        }
        self.mode.execute_move()
    }

    pub fn backend(&mut self) -> &mut B {
        &mut self.backend
    }
}

#[test]
#[cfg(feature = "discord")]
fn test() {
    use crate::defaults::normal::Default8x8;
    use crate::discord::Discord;
    let mut game: Game<Default8x8, Discord> = Game::new();
    game.next_move()
}