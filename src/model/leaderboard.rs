use std::ops::Add;

use chrono::Utc;
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

use crate::db::Db;
use crate::model::leaderboard_level_info::LeaderboardLevelInfo;

pub const MAX_PLAYERS: usize = 10;

#[derive(Serialize, Deserialize, Clone)]
pub struct LeaderboardPlayer {
    id: ObjectId,
    score: i32,
}

impl LeaderboardPlayer {
    pub fn get_id(&self) -> &ObjectId {
        &self.id
    }

    pub fn get_score(&self) -> i32 {
        self.score
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Leaderboard {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    id: Option<ObjectId>,
    players: Vec<LeaderboardPlayer>,
    starts_at: chrono::DateTime<Utc>,
    ends_at: chrono::DateTime<Utc>,
}

impl Leaderboard {
    pub fn new() -> Self {
        Self {
            id: None,
            players: vec![],
            starts_at: Utc::now().add(chrono::TimeDelta::seconds(30)),
            ends_at: Utc::now()
                .add(chrono::TimeDelta::seconds(30))
                .add(chrono::TimeDelta::minutes(60)),
        }
    }

    pub fn add_player(&mut self, player_id: ObjectId) {
        self.players.push(LeaderboardPlayer {
            id: player_id,
            score: 0,
        });
    }

    pub fn get_id(&self) -> &Option<ObjectId> {
        &self.id
    }

    pub fn get_players(&self) -> &[LeaderboardPlayer] {
        &self.players
    }

    pub fn has_player(&self, player_id: &ObjectId) -> bool {
        self.players
            .iter()
            .any(|player| player.get_id() == player_id)
    }

    pub async fn get_level_info(&self, db: &Db) -> LeaderboardLevelInfo {
        LeaderboardLevelInfo::new(self, db).await
    }

    pub fn is_full(&self) -> bool {
        self.players.len() < MAX_PLAYERS
    }

    pub fn get_ends_at_timestamp(&self) -> String {
        self.ends_at.to_rfc3339()
    }

    pub fn has_started(&self) -> bool {
        Utc::now() > self.starts_at
    }
}
