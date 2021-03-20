// use crate::{Backend};
// use std::error::Error;
//
// pub struct Terminal {
//     queue: Vec<String>,
// }
//
// impl Backend for Terminal {
//     fn new() -> Self {
//         Self { queue: vec![] }
//     }
//
//     fn receive(&self) -> Result<Option<String>, String> {
//         let mut buffer = String::new();
//         std::io::stdin().read_line(&mut buffer)?;
//         Ok(Some(buffer))
//     }
//
//     fn send(&mut self, msg: String) -> Result<Option<String>, String> {
//         self.queue.push(msg);
//
//         Ok(None)
//     }
// }
