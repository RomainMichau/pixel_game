use std::time::Duration;

use chrono::{DateTime, Utc};
use derive_more::Display;
use serde::{Deserialize, Serialize};

pub trait DataSourcePort {
    fn get_grid(&self) -> &Vec<PixelColor>;
    fn set_pixel(&mut self, pixel_id: usize, pixel_color: PixelColor) -> Result<(), PixelGameError>;
    fn init_grid(&mut self, width: usize, height: usize, init_color: PixelColor);
    fn create_new_player(&mut self, player_name: String) -> &Player;
    fn update_last_play(&mut self, player_id: usize, timestamp: DateTime<Utc>) -> Result<(), PixelGameError>;
    fn get_player(&self, player_id: usize) -> Option<&Player>;
}


#[derive(Debug, Display, Clone, Serialize)]
pub enum PixelGameError {
    PlayerNotFound,
    InvalidCoordinates,
    PlayerNeedToWaitSeconds(usize)
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

#[derive(Clone)]
pub struct Player {
    pub id: usize,
    pub name: String,
    pub last_played: Option<DateTime<Utc>>,
}


pub trait PixelGame {
    fn set_pixel(&mut self, pixel_id: usize, player_id: usize, color: PixelColor) -> Result<(), PixelGameError>;
    fn get_board(&self) -> &Vec<PixelColor>;
    fn get_width(&self) -> usize;
    fn get_height(&self) -> usize;
    fn create_new_player(&mut self, player_name: String) -> &Player;
}

pub(crate) struct PixelGameImpl {
    data_source: Box<dyn DataSourcePort + Send>,
    width: usize,
    height: usize,
    turn_duration: chrono::Duration,
}

impl PixelGameImpl {
    pub(crate) fn new(width: usize, height: usize, init_color: PixelColor, turn_duration: Duration, mut data_source: Box<dyn DataSourcePort + Send>) -> Self {
        data_source.init_grid(width, height, init_color);
        PixelGameImpl {
            width,
            height,
            turn_duration: chrono::Duration::from_std(turn_duration).unwrap(),
            data_source,
        }
    }
}

impl PixelGame for PixelGameImpl {
    fn set_pixel(&mut self, pixel_id: usize, player_id: usize, color: PixelColor) -> Result<(), PixelGameError> {
        if pixel_id >= self.width * self.height {
            return Err(PixelGameError::InvalidCoordinates);
        }
        let player = match self.data_source.get_player(player_id) {
            Some(player) => player,
            None => return Err(PixelGameError::PlayerNotFound),
        };
        if let Some(last_played) = player.last_played {
            if Utc::now().signed_duration_since(last_played) < self.turn_duration {
                let wait = (self.turn_duration - Utc::now().signed_duration_since(last_played)).to_std().unwrap();
                return Err(PixelGameError::PlayerNeedToWaitSeconds(wait.as_secs() as usize))
            }
        }
        self.data_source.update_last_play(player_id, Utc::now())?;
        self.data_source.set_pixel(pixel_id, color)?;
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

    fn create_new_player(&mut self, player_name: String) -> &Player {
        self.data_source.create_new_player(player_name)
    }
}

