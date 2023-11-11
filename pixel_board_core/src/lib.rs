use std::time::Duration;
use crate::board::{PixelGame, PixelColor, DataSourcePort};

pub mod board;

//method allowing library users to create a mutable new board with the specified width and height.
pub fn init(width: usize, height: usize, init_color: PixelColor, duration: Duration, data_source: Box<dyn DataSourcePort>) -> Box<dyn PixelGame> {
    Box::new(board::PixelGameImpl::new(width, height, init_color, duration, data_source))
}