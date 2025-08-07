use std::collections::VecDeque;
use pelican_game_engine::Sprite;
use pelican_ui::Context;
use pelican_ui_std::Offset;

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

    pub fn get_deque_of_id(&self) -> String {
        let id_string = &self.deque_of_id.back().unwrap();
        id_string.to_string()
    }

    pub fn add_to_deque_of_id(&mut self) {
        if let Some(last) = self.deque_of_id.back() {
            self.deque_of_id.push_back(last + 1);
        }
        self.deque_of_id.pop_front();
    }

    pub fn terrain_block_sprite_generator(ctx: &mut Context, id_string: String) -> Sprite {
        Sprite::new(ctx, &id_string, "green_terrain", (10.0, 10.0), (Offset::End, Offset::End))
        
    }
}

