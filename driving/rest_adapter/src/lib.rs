mod openapi;
mod controller;

use std::sync::Mutex;

use actix_web::{App, HttpServer};
pub use actix_web::main;
use actix_web::web::Data;
use utoipa::{OpenApi};
use utoipa_swagger_ui::SwaggerUi;

use pixel_board_core::board::PixelGame;


pub async fn start(game: Box<dyn PixelGame + Send + 'static>) -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();
    let data: Data<Mutex<Box<dyn PixelGame + Send>>> = Data::new(Mutex::new(game));
    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .service(controller::hello)
            .service(controller::set_pixel)
            .service(controller::get_board)
            .service(controller::create_player)
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-docs/openapi.json", openapi::ApiDoc::openapi()),
            )
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}

