use axum::{Extension, Json};
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use mongodb::bson::oid::ObjectId;
use serde::Serialize;

use crate::App;
use crate::model::{Leaderboard, Player};

#[derive(Serialize)]
struct Output {
    leaderboard_id: ObjectId,
    ends_at: String,
}

pub async fn join(State(app): State<App>, Extension(player): Extension<Player>) -> Response {
    // Check first if there are any suitable leaderboards looking for new players
    if let Some(mut leaderboard) = find_suitable_leaderboard(&player, &app).await {
        // Add the player to it
        leaderboard.add_player(player.get_id().unwrap().clone());

        // Compile required output
        let output = Output {
            leaderboard_id: leaderboard.get_id().unwrap().clone(),
            ends_at: leaderboard.get_ends_at_timestamp(),
        };

        // Save the updated leaderboard
        app.db
            .leaderboard_model
            .update_leaderboard(&leaderboard)
            .await;

        return Json(output).into_response();
    }

    StatusCode::ACCEPTED.into_response()
}

async fn find_suitable_leaderboard(player: &Player, app: &App) -> Option<Leaderboard> {
    // Find the first not full and not in progress leaderboard where the player's level fits inside
    // the current level range on the leaderboard

    // This is a bad way to do this. Preferably use mongo to try to find a match, would need more
    // time to figure out a better schema and then build some kind of aggregation pipeline.

    let leaderboards = app.db.leaderboard_model.get_leaderboards().await;

    let leaderboards: Vec<Leaderboard> = leaderboards
        .into_iter()
        .filter(|leaderboard| !leaderboard.is_full() && !leaderboard.has_started())
        .collect();

    for leaderboard in leaderboards {
        let info = leaderboard.get_level_info(&app.db).await;
        if info.is_in_range(player.get_level()) {
            return Some(leaderboard);
        }
    }

    None
}
