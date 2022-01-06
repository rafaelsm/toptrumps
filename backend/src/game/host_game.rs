use warp::{http, Filter};
use super::models::GameRoom;

pub fn new_game() {
    let create_game_path = warp::path("/new").map(||"Creating new game");
    // return create_game_path
}

pub fn json_body() -> impl Filter<Extract = (GameRoom,), Error = warp::Rejection> + Clone {
    let limit = 1024 * 16;
    warp::body::content_length_limit(limit).and(warp::body::json())
}

pub async fn create_room(game_room: GameRoom) -> Result<impl warp::Reply, warp::Rejection> {
    let reply = format!("game room -{}- created", game_room.name);
    Ok(warp::reply::with_status(reply, http::StatusCode::CREATED))
}