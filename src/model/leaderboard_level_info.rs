use mongodb::bson::oid::ObjectId;

use crate::db::Db;
use crate::model::Leaderboard;

pub struct LeaderboardLevelInfo {
    pub average_player_level: i32,
    pub min_player_level: i32,
    pub max_player_level: i32,
}

impl LeaderboardLevelInfo {
    pub async fn new(leaderboard: &Leaderboard, db: &Db) -> Self {
        let player_levels = get_player_levels(leaderboard, db).await;
        Self {
            average_player_level: calculate_average_player_level(&player_levels),
            min_player_level: calculate_min_player_level(&player_levels),
            max_player_level: calculate_max_player_level(&player_levels),
        }
    }

    pub fn is_in_range(&self, level: i32) -> bool {
        self.min_player_level >= level && level <= self.max_player_level
    }
}

fn calculate_average_player_level(player_levels: &[i32]) -> i32 {
    let total_player_level: i32 = player_levels.iter().sum();

    total_player_level / player_levels.len() as i32
}

fn calculate_min_player_level(player_levels: &[i32]) -> i32 {
    let min_player_level = player_levels.into_iter().min();

    *(min_player_level.unwrap_or(&0))
}

fn calculate_max_player_level(player_levels: &[i32]) -> i32 {
    let max_player_level = player_levels.into_iter().max();

    *(max_player_level.unwrap_or(&0))
}

fn get_player_ids(leaderboard: &Leaderboard) -> Vec<ObjectId> {
    leaderboard
        .get_players()
        .iter()
        .map(|player| player.get_id())
        .cloned()
        .collect()
}

async fn get_player_levels(leaderboard: &Leaderboard, db: &Db) -> Vec<i32> {
    let player_ids = get_player_ids(leaderboard);
    let players = futures::future::join_all(
        player_ids
            .iter()
            .map(|player_id| async { db.player_model.find_player(player_id).await }),
    )
    .await;

    players
        .into_iter()
        .flatten()
        .map(|player| player.get_level())
        .collect()
}
