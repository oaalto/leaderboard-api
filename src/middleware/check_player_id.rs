use axum::extract::{Query, Request, State};
use axum::http::StatusCode;
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use mongodb::bson::oid::ObjectId;
use serde::Deserialize;

use crate::App;

#[derive(Deserialize)]
pub struct PlayerIdQuery {
    value: String,
}

/// Checks that the provided player id is valid and a player exists with the id
pub async fn check_player_id(
    Query(player_id): Query<PlayerIdQuery>,
    State(app): State<App>,
    mut request: Request,
    next: Next,
) -> Response {
    if player_id.value.is_empty() {
        return StatusCode::BAD_REQUEST.into_response();
    }

    let player_id = match ObjectId::parse_str(player_id.value) {
        Ok(id) => id,
        Err(_) => {
            return StatusCode::BAD_REQUEST.into_response();
        }
    };

    let player = app.db.player_model.find_player(&player_id).await;
    match player {
        None => StatusCode::BAD_REQUEST.into_response(),
        Some(player) => {
            request.extensions_mut().insert(player);
            next.run(request).await
        }
    }
}
