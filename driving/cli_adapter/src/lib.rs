use colored::Colorize;

use pixel_board_core::board;
use pixel_board_core::board::{PixelGame, Player};

pub fn start_game(mut game: Box<dyn PixelGame>) {
    // let in_memory_adapter = in_memory_adapter::init(100, 100, board::PixelColor::White);
    // let mut game = pixel_board_core::init(100, 100, board::PixelColor::White,
    //                                       std::time::Duration::from_secs(100), in_memory_adapter);
    print_board(&game);
    let player = game.create_new_player("romain".to_string());

    while {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        if input == "quit" {
            false
        } else {
            let mut input = input.split_whitespace();
            let x = input.next().unwrap().parse::<usize>().unwrap();
            let y = input.next().unwrap().parse::<usize>().unwrap();
            let color = match input.next().unwrap() {
                "green" => board::PixelColor::Green,
                "red" => board::PixelColor::Red,
                "white" => board::PixelColor::White,
                "yellow" => board::PixelColor::Yellow,
                "black" => board::PixelColor::Black,
                "blue" => board::PixelColor::Blue,
                _ => board::PixelColor::Green
            };
            let pixel_id = get_pixel_id(&game, x, y);
            match edit_pixel(&mut game, pixel_id, color, player.id) {
                Ok(_) => {
                    print_board(&game);
                }
                Err(e) => match e {
                    board::PixelGameError::PlayerNotFound => println!("Player not found"),
                    board::PixelGameError::InvalidCoordinates => println!("Invalid coordinates"),
                    board::PixelGameError::PlayerNeedToWaitSeconds(remaining) => println!("Player already played, remaining time: {:?}", remaining),
                }
            }
            true
        }
    } {}
}

fn edit_pixel(board: &mut Box<dyn PixelGame>, pixel_id: usize, color: board::PixelColor, player_id: usize) -> Result<(), board::PixelGameError> {
    board.set_pixel(pixel_id, player_id, color)
}

fn print_board(board: &Box<dyn PixelGame>) {
    let char = "▣";
    for y in 0..board.get_height() {
        for x in 0..board.get_width() {
            match board.get_board()[y * board.get_width() + x] {
                board::PixelColor::Green => print!("{}", char.green()),
                board::PixelColor::Red => print!("{}", char.red()),
                board::PixelColor::White => print!("{}", char.white()),
                board::PixelColor::Yellow => print!("{}", char.yellow()),
                board::PixelColor::Black => print!("{}", char.black()),
                board::PixelColor::Blue => print!("{}", char.blue())
            }
        }
        println!();
    }
    println!();
}

fn get_pixel_id(board: &Box<dyn PixelGame>, x: usize, y: usize) -> usize {
    y * board.get_width() + x
}


