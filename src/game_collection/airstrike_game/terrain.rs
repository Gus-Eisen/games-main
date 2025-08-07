use std::collections::VecDeque;
use pelican_game_engine::Sprite;

#[derive(Debug, Default, Clone)]
pub struct Terrain {
    // sprite: Sprite
    pub deque_of_id: VecDeque<u64>
}
impl Terrain {
    pub fn new() -> Self {
        Self {
            deque_of_id: VecDeque::from(vec![0])
        }
    }

    pub fn add_to_deque_of_id(&mut self) {
        if let Some(last) = self.deque_of_id.back() {
            self.deque_of_id.push_back(last + 1);
        }
        self.deque_of_id.pop_front();
    }
}

