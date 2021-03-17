use crate::{Backend};
use std::error::Error;

pub struct Discord {
    queue: Vec<String>,
}

impl Backend for Discord {
    fn new() -> Self {
        Self { queue: vec![] }
    }

    fn receive(&self) -> Result<Option<String>, String> {
        unimplemented!()
    }

    fn send(&mut self, msg: String) -> Result<Option<String>, String> {
        self.queue.push(msg);

        Ok(None)
    }
}
