use std::collections::HashMap;
use std::time::Instant;

use pixel_board_core::board::{DataSourcePort, PixelColor, PixelGameError, Player};

pub fn new() -> Box<dyn DataSourcePort + Send> {
    return Box::new(InMemoryAdapter {
        grid: Vec::new(),
        players: HashMap::new(),
    });
}

struct InMemoryAdapter {
    grid: Vec<PixelColor>,
    players: HashMap<usize, Player>,
}

impl DataSourcePort for InMemoryAdapter {
    fn get_grid(&self) -> &Vec<PixelColor> {
        return &self.grid;
    }

    fn set_pixel(&mut self, pixel_id: usize, pixel_color: PixelColor) -> Result<(), PixelGameError> {
        self.grid[pixel_id] = pixel_color;
        return Ok(());
    }

    fn init_grid(&mut self, width: usize, height: usize, init_color: PixelColor) {
        self.grid = vec![init_color; width * height];
    }

    fn create_new_player(&mut self, player_name: String) -> usize {
        let id = self.players.len() + 1;
        self.players.insert(id, Player {
            id,
            name: player_name,
            last_played: None,
        });
        return id;
    }

    fn update_last_play(&mut self, player_id: usize, timestamp: Instant) -> Result<(), PixelGameError> {
        let player = self.players.get_mut(&player_id).ok_or(PixelGameError::PlayerNotFound)?;
        player.last_played = Some(timestamp);
        return Ok(());
    }


    fn get_player(&self, player_id: usize) -> Option<&Player> {
        return self.players.get(&player_id);
    }
}
