use axum::Extension;
use axum::extract::{Request, State};
use axum::http::StatusCode;
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};

use crate::App;
use crate::model::Player;

/// Checks if the player is already on a leaderboard
pub async fn check_for_leaderboard(
    Extension(player): Extension<Player>,
    State(app): State<App>,
    request: Request,
    next: Next,
) -> Response {
    if app
        .db
        .leaderboard_model
        .find_leaderboard_for_player(&player.get_id().unwrap())
        .await
        .is_some()
    {
        return StatusCode::CONFLICT.into_response();
    }

    next.run(request).await
}
