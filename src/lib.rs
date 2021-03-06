use glucose::linear::vec::Point;

pub mod cache;
pub mod defaults;
#[cfg(feature = "discord")]
pub mod discord;
#[cfg(feature = "terminal")]
pub mod terminal;

#[derive(Debug, Copy, Clone)]
pub struct Move<const N: usize> {
    pub player_id: i32,
    pub piece_id: i32,
    pub from: Option<Point<i32, { N }>>,
    pub to: Point<i32, { N }>,
}

#[derive(Debug, Copy, Clone)]
pub struct RenderMove2D {
    pub player_id: i32,
    pub piece_id: i32,
    pub pos: Point<i32, 2>,
}

#[derive(Debug, Clone)]
pub struct Player {
    pub id: i32,
}

impl Player {
    pub const fn new(id: i32) -> Self {
        Self { id }
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
        Self {
            player,
            id,
            sub_id: None,
            ty,
            pos,
        }
    }

    pub fn new_with_sub(
        player: i32,
        id: i32,
        sub_id: i32,
        ty: i32,
        pos: Point<i32, { N }>,
    ) -> Self {
        Self {
            player,
            id,
            sub_id: Some(sub_id),
            ty,
            pos,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Board<const DIMS: usize> {
    start: Point<i32, { DIMS }>,
    size: [i32; DIMS],
}

impl<const DIMS: usize> Board<{ DIMS }> {
    pub fn new(start: Point<i32, { DIMS }>, size: [i32; DIMS]) -> Self {
        Self { start, size }
    }
}

#[derive(Debug, Clone)]
pub enum PlayerSwap {
    NextUp,
    NextDown,
    Same,
    Custom(i32),
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

    fn rendered_board(&self) -> String;

    fn next_player(&self) -> PlayerSwap;
}

pub trait Backend {
    type Id;

    fn new() -> Self;

    fn receive(&mut self) -> Result<Option<String>, String>;

    fn send(&mut self, msg: String) -> Result<Option<String>, String>;

    fn player_to_backend(&self, id: i32) -> Self::Id;

    fn backend_to_player(&self, id: Self::Id) -> i32;
}

#[derive(Debug, Clone)]
pub struct Game<M: Mode, B: Backend> {
    pub mode: M,
    pub backend: B,
    pub players: Vec<Player>,
    pub current_player: (i32, usize),
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
            current_player: (M::STARTING_PLAYER, 0),
        }
    }

    pub fn next_move(&mut self) {
        let mut input = self.backend.receive();
        while input.is_err() {
            match self.backend.send(String::from("Wrong Input")) {
                Ok(_) => {}
                Err(err) => {
                    panic!(format!("{}", err))
                }
            };
            input = self.backend.receive();
        }
        self.mode.next_move(input.unwrap().unwrap(), self.current_player.0).unwrap();
        self.mode.execute_move(self.current_player.0);
        self.swap_player(self.mode.next_player())
    }

    fn swap_player(&mut self, swap: PlayerSwap) {
        match swap {
            PlayerSwap::NextUp => {
                let maybe_player = self.players.get(self.current_player.1 + 1);
                match maybe_player {
                    Some(player) => self.current_player = (player.id, self.current_player.1 + 1),
                    None => self.current_player = (self.players[0].id, 0),
                }
            }
            PlayerSwap::NextDown => {
                let maybe_player = self.players.get(self.current_player.1 - 1);
                match maybe_player {
                    Some(player) => self.current_player = (player.id, self.current_player.1 - 1),
                    None => self.current_player = (self.players[M::PLAYERS - 1].id, M::PLAYERS - 1),
                }
            }
            PlayerSwap::Same => {}
            PlayerSwap::Custom(next) => {
                self.current_player = self
                    .players
                    .iter()
                    .enumerate()
                    .find(|(_, player)| player.id == next)
                    .map(|(idx, player)| (player.id, idx))
                    .unwrap()
            }
        }
    }
}

#[test]
#[cfg(feature = "discord")]
fn test() {
    use crate::defaults::normal::Default8x8;
    use crate::discord::Discord;
    let mut game: Game<Default8x8, Discord> = Game::new();
    println!("{}", game.mode.rendered_board());
    game.backend.send(String::from("1 2 -> 2 2"));
    game.next_move();
    game.backend.send(String::from("6 3 -> 5 3"));
    game.next_move();
    game.backend.send(String::from("2 2 -> 3 2"));
    game.next_move();
    game.backend.send(String::from("5 3 -> 4 3"));
    game.next_move();
    game.backend.send(String::from("3 2 -> 4 3"));
    game.next_move();
    println!("{}", game.mode.rendered_board());
}
