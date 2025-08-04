use std::collections::VecDeque;
use pelican_game_engine::Sprite;

pub struct Terrain {
    // sprite: Sprite
    deque_of_id: VecDeque<u8>
}
impl Terrain {
    fn new(id: &str) -> Self {
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

