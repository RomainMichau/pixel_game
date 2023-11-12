use std::sync::Mutex;

use actix_web::{get, HttpRequest, HttpResponse, post, Responder, web};
use actix_web::body::BoxBody;
use actix_web::http::header::ContentType;
use actix_web::web::Data;
use chrono::{DateTime, Utc};
use derive_more::{Display, Error};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

use pixel_board_core::board::{PixelGame, PixelGameError};

#[derive(Clone)]
#[derive(Debug)]
#[derive(Deserialize)]
#[derive(Serialize)]
#[derive(ToSchema)]
pub(crate) enum PixelColor {
    Green,
    Red,
    White,
    Yellow,
    Black,
    Blue,
}


#[derive(Debug, Display, Error)]
pub(crate) struct PixelGameErrorWrapper {
    error: PixelGameError,
}

impl PixelGameErrorWrapper {
    pub fn new(error: PixelGameError) -> Self {
        PixelGameErrorWrapper {
            error,
        }
    }
}

#[derive(Serialize)]
struct PixelGameErrorResponse {
    error_message: String,
    error: PixelGameError,
}

impl actix_web::error::ResponseError for PixelGameErrorWrapper {
    fn error_response(&self) -> HttpResponse {
        match self.error {
            PixelGameError::PlayerNotFound => {
                let message = "Player not found";
                HttpResponse::NotFound()
                    .content_type(ContentType::json())
                    .body(serde_json::to_string(&PixelGameErrorResponse {
                        error_message: message.to_string(),
                        error: self.error.clone(),
                    }).unwrap())
            }
            PixelGameError::InvalidCoordinates => {
                let message = "Invalid pixel id";
                HttpResponse::BadRequest()
                    .content_type(ContentType::json())
                    .body(serde_json::to_string(&PixelGameErrorResponse {
                        error_message: message.to_string(),
                        error: self.error.clone(),
                    }).unwrap())
            }
            PixelGameError::PlayerNeedToWaitSeconds(e) => {
                let message = format!("Player already played, wait {:#?}", e);
                HttpResponse::BadRequest()
                    .content_type(ContentType::json())
                    .body(serde_json::to_string(&PixelGameErrorResponse {
                        error_message: message.to_string(),
                        error: self.error.clone(),
                    }).unwrap())
            }
        }
    }
}

#[derive(Deserialize, ToSchema, IntoParams)]
pub(crate) struct CreatePlayerRequest {
    name: String,
}

#[derive(Clone)]
#[derive(Serialize)]
struct PlayerResponse {
    pub id: usize,
    pub name: String,
    pub last_played: Option<DateTime<Utc>>,
}

impl Responder for PlayerResponse {
    type Body = BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(serde_json::to_string(&self).unwrap())
    }
}

impl From<pixel_board_core::board::Player> for PlayerResponse {
    fn from(value: pixel_board_core::board::Player) -> Self {
        PlayerResponse {
            id: value.id,
            name: value.name,
            last_played: value.last_played,
        }
    }
}


impl Into<pixel_board_core::board::PixelColor> for PixelColor {
    fn into(self) -> pixel_board_core::board::PixelColor {
        match self {
            PixelColor::Green => pixel_board_core::board::PixelColor::Green,
            PixelColor::Red => pixel_board_core::board::PixelColor::Red,
            PixelColor::White => pixel_board_core::board::PixelColor::White,
            PixelColor::Yellow => pixel_board_core::board::PixelColor::Yellow,
            PixelColor::Black => pixel_board_core::board::PixelColor::Black,
            PixelColor::Blue => pixel_board_core::board::PixelColor::Blue,
        }
    }
}

#[derive(Deserialize, ToSchema, IntoParams, Debug)]
pub(crate) struct SetPixelRequest {
    color: PixelColor,
    player_id: usize,
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}


#[utoipa::path(
get,
responses(
(status = 200, description = "Return the board", body = Vec < PixelColor >),
(status = NOT_FOUND, description = "Pet was not found")
),
)]
#[get("/board")]
pub async fn get_board(data: Data<Mutex<Box<dyn PixelGame + Send>>>) -> actix_web::error::Result<impl Responder> {
    Ok(web::Json(data.lock().unwrap().get_board().clone()))
}

#[utoipa::path(
responses(
(status = 200, description = "Pixel was modified"),
(status = NOT_FOUND, description = "Pet was not found")
),
)]
#[post("/pixel/{pixel_id}")]
async fn set_pixel(pixel_id: web::Path<(usize, )>, req: web::Json<SetPixelRequest>, data: Data<Mutex<Box<dyn PixelGame + Send>>>) -> actix_web::Result<impl Responder, PixelGameErrorWrapper> {
    data.lock().unwrap().set_pixel(pixel_id.into_inner().0, req.player_id, req.color.clone().into())
        .map_err(|e| PixelGameErrorWrapper::new(e))?;
    Ok(HttpResponse::Created().body(""))
}

#[utoipa::path(
responses(
(status = 201, description = "Player was created"),
(status = CONFLICT, description = "Player name already used")
),
)]
#[post("/player")]
async fn create_player(data: Data<Mutex<Box<dyn PixelGame + Send>>>, player_name: web::Json<CreatePlayerRequest>) -> actix_web::Result<PlayerResponse, PixelGameErrorWrapper> {
    let mut game = data.lock().unwrap();
    let player = game.create_new_player(player_name.name.clone());
    Ok(PlayerResponse::from(player.clone()))
}