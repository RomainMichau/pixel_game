use pixel_board_core::board;

fn main() {
    let in_memory_adapter = in_memory_adapter::new();
    let game = pixel_board_core::init(100, 100, board::PixelColor::White,
                                          std::time::Duration::from_secs(10), in_memory_adapter);
    cli_adapter::start_game(game);
}
