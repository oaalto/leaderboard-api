use axum::Router;
use axum::routing::post;
use mongodb::bson::oid::ObjectId;
use tokio::net::TcpListener;
use tokio::sync::mpsc;
use tokio::sync::mpsc::UnboundedSender;
use tracing::info;

use leaderboard::{App, handler, middleware};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    let app = App::new().await.unwrap();

    let routes = Router::new()
        .route("/leaderboard/join", post(handler::join))
        .route_layer(axum::middleware::from_fn_with_state(
            app.clone(),
            middleware::check_for_leaderboard,
        ))
        .route_layer(axum::middleware::from_fn_with_state(
            app.clone(),
            middleware::check_player_id,
        ))
        .with_state(app);

    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    info!("Server started, listening: {:?}\n", listener.local_addr());
    axum::serve(listener, routes.into_make_service())
        .await
        .unwrap();
}

/// This isn't how I would do processing behind a REST API. I would build for example completely
/// new application to do any processing that have a time component. Communication would happen
/// with a queue of some sort.
fn start_background_process() -> UnboundedSender<ObjectId> {
    let (tx, mut rx) = mpsc::unbounded_channel::<ObjectId>();
    tokio::spawn(async move {
        loop {
            if let Some(player_id) = rx.recv().await {
            } else {
                return;
            }
        }
    });

    tx
}
