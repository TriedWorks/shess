use crate::{Backend, Player};
use std::error::Error;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Discord {
    pub queue: Vec<String>,
    pub player_discord: HashMap<i32, i32>,
    pub discord_player: HashMap<i32, i32>
}

impl Discord {
    pub fn set_player(&mut self, shess_id: i32, discord_id: i32) {
        self.discord_player.insert(discord_id, shess_id);
        self.player_discord.insert(shess_id, discord_id);
    }
}

impl Backend for Discord {
    type Id = i32;

    fn new() -> Self {
        Self { queue: vec![], player_discord: Default::default(), discord_player: Default::default() }
    }

    fn receive(&mut self) -> Result<Option<String>, String> {
        let input = match self.queue.pop() {
            None => return Err(String::from("Empty")),
            Some(input) => Some(input),
        };

        Ok(input)
    }

    fn send(&mut self, msg: String) -> Result<Option<String>, String> {
        self.queue.push(msg);

        Ok(None)
    }

    fn player_to_backend(&self, id: i32) -> Self::Id {
        *self.player_discord.get(&id).unwrap()
    }

    fn backend_to_player(&self, id: Self::Id) -> i32 {
        *self.discord_player.get(&id).unwrap()
    }
}
