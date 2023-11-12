use pixel_board_core::board;

#[actix_web::main]
async fn main() {
    let in_memory_adapter = in_memory_adapter::new();
    let game = pixel_board_core::init(10, 10, board::PixelColor::White,
                                          std::time::Duration::from_secs(10), in_memory_adapter);
    rest_adapter::start(game).await.unwrap();
}
