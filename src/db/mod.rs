use mongodb::Client;

use crate::db::model::{LeaderboardModel, PlayerModel};
use crate::model::{Leaderboard, Player};

mod model;

const CONNECTION_URL: &str = "mongodb://localhost:27017/";

#[derive(Clone)]
pub struct Db {
    pub player_model: PlayerModel,
    pub leaderboard_model: LeaderboardModel,
}

impl Db {
    pub async fn new() -> mongodb::error::Result<Self> {
        let client = Client::with_uri_str(CONNECTION_URL).await?;
        let database = client.database("some_db");

        Ok(Self {
            player_model: PlayerModel::new(database.collection::<Player>("player")),
            leaderboard_model: LeaderboardModel::new(
                database.collection::<Leaderboard>("leaderboard"),
            ),
        })
    }
}
