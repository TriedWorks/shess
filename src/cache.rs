use crate::Move;

#[derive(Debug, Copy, Clone)]
pub struct MoveCache<const N: usize> {
    pub cache: Vec<Move<{ N }>>
}

impl<const N: usize> MoveCache<{ N }> {
    pub fn new() -> Self {
        Self { cache: vec![] }
    }

    pub fn new_with_capacity(capacity: usize) -> Self {
        Self {
            cache: Vec::with_capacity(capacity)
        }
    }
}