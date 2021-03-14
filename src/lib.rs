use glucose::linear::vec::Point;

pub mod defaults;
#[cfg(feature = "discord")]
pub mod discord;
#[cfg(feature = "terminal")]
pub mod terminal;

pub struct Player {
    pieces: Vec<i32>
}

impl Player {
    pub const fn new(pieces: Vec<i32>) -> Self {
        Self { pieces }
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
    size: [i32; DIMS],
    board: Vec<Option<i32>>
}

impl<const DIMS: usize> Board<{ DIMS }> {
    pub fn new(size: [i32; DIMS]) -> Self {
        Self {
            size,
            board: vec![]
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
            self.backend.send(String::from("Wrong Input"));
            input = self.backend.receive();
        }
        self.mode.next_move(input.unwrap().unwrap(), self.current_player);
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