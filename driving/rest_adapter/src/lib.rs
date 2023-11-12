use std::sync::Mutex;

use actix_web::{App, get, HttpResponse, HttpServer, post, Responder, web};
pub use actix_web::main;
use actix_web::web::Data;
use serde::Deserialize;

use pixel_board_core::board::{PixelColor, PixelGame};

#[derive(Deserialize)]
pub(crate) struct SetPixelRequest {
    x: usize,
    y: usize,
    color: PixelColor,
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/board")]
async fn get_board(data: Data<Mutex<Box<dyn PixelGame + Send>>>) -> actix_web::error::Result<impl Responder> {
    Ok(web::Json(data.lock().unwrap().get_board().clone()))
}


#[post("/pixel")]
async fn echos(req: web::Query<SetPixelRequest>, data: Data<Mutex<Box<dyn PixelGame + Send>>>) -> impl Responder {
    data.lock().unwrap().set_pixel(req.x, req.y, 0, req.color.clone().into()).expect("TODO: panic message");
    HttpResponse::Ok()
}


pub async fn start(game: Box<dyn PixelGame + Send + 'static>) -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();
    let data: Data<Mutex<Box<dyn PixelGame + Send>>> = Data::new(Mutex::new(game));
    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .service(hello)
            .service(echos)
            .service(get_board)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}