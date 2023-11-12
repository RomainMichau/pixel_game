use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(paths(
crate::controller::get_board,
crate::controller::set_pixel,
crate::controller::create_player,
)
, components(schemas(crate::controller::SetPixelRequest, crate::controller::PixelColor, crate::controller::CreatePlayerRequest)))]
pub(crate) struct ApiDoc;


