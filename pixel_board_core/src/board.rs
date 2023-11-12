use std::time::{Duration, Instant};
use serde::{Deserialize, Serialize};

pub trait DataSourcePort {
    fn get_grid(&self) -> &Vec<PixelColor>;
    fn set_pixel(&mut self, pixel_id: usize, pixel_color: PixelColor) -> Result<(), PixelGameError>;
    fn init_grid(&mut self, width: usize, height: usize, init_color: PixelColor);
    fn create_new_player(&mut self, player_name: String) -> usize;
    fn update_last_play(&mut self, player_id: usize, timestamp: Instant) -> Result<(), PixelGameError>;
    fn get_player(&self, player_id: usize) -> Option<&Player>;
}

#[derive(Debug)]
pub enum PixelGameError {
    PlayerNotFound,
    InvalidCoordinates,
    PlayerAlreadyPlayed(Duration),
}

#[derive(Clone)]
#[derive(Debug)]
#[derive(Deserialize)]
#[derive(Serialize)]
pub enum PixelColor {
    Green,
    Red,
    White,
    Yellow,
    Black,
    Blue,
}

pub struct Player {
    pub id: usize,
    pub name: String,
    pub last_played: Option<Instant>,
}


pub trait PixelGame {
    fn set_pixel(&mut self, x: usize, y: usize, player_id: usize, color: PixelColor) -> Result<(), PixelGameError>;
    fn get_board(&self) -> &Vec<PixelColor>;
    fn get_width(&self) -> usize;
    fn get_height(&self) -> usize;
    fn create_new_player(&mut self, player_name: String) -> usize;
}

pub(crate) struct PixelGameImpl {
    data_source: Box<dyn DataSourcePort + Send>,
    width: usize,
    height: usize,
    turn_duration: Duration,
}

impl PixelGameImpl {
    pub(crate) fn new(width: usize, height: usize, init_color: PixelColor, turn_duration: Duration, mut data_source: Box<dyn DataSourcePort + Send>) -> Self {
        data_source.init_grid(width, height, init_color);
        PixelGameImpl {
            width,
            height,
            turn_duration,
            data_source,
        }
    }


    fn get_pixel_id(&self, x: usize, y: usize) -> usize {
        x + y * self.width
    }
}

impl PixelGame for PixelGameImpl {
    fn set_pixel(&mut self, x: usize, y: usize, player_id: usize, color: PixelColor) -> Result<(), PixelGameError> {
        if x >= self.width || y >= self.height {
            return Err(PixelGameError::InvalidCoordinates);
        }
        let player = match self.data_source.get_player(player_id) {
            Some(player) => player,
            None => return Err(PixelGameError::PlayerNotFound),
        };
        if let Some(last_played) = player.last_played {
            if last_played.elapsed() < self.turn_duration {
                return Err(PixelGameError::PlayerAlreadyPlayed(self.turn_duration - last_played.elapsed()));
            }
        }
        self.data_source.update_last_play(player_id, Instant::now())?;
        self.data_source.set_pixel(self.get_pixel_id(x, y), color)?;
        return Ok(());
    }


    fn get_board(&self) -> &Vec<PixelColor> {
        &self.data_source.get_grid()
    }

    fn get_width(&self) -> usize {
        self.width
    }

    fn get_height(&self) -> usize {
        self.height
    }

    fn create_new_player(&mut self, player_name: String) -> usize {
        self.data_source.create_new_player(player_name)
    }
}

