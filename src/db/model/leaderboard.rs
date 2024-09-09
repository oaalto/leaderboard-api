use futures::stream::TryStreamExt;
use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;
use mongodb::Collection;

use crate::model::Leaderboard;

#[derive(Clone)]
pub struct LeaderboardModel {
    db: Collection<Leaderboard>,
}

impl LeaderboardModel {
    pub fn new(db: Collection<Leaderboard>) -> Self {
        Self { db }
    }

    pub async fn find_leaderboard(&self, id: &ObjectId) -> Option<Leaderboard> {
        let result = self
            .db
            .find_one(doc! {
                "_id": id
            })
            .await;

        match result {
            Err(e) => {
                eprintln!("Failed to fetch leaderboard {:?}: {}", id, e);
                None
            }
            Ok(leaderboard) => leaderboard,
        }
    }

    pub async fn add_leaderboard(&self, leaderboard: &Leaderboard) {
        let result = self.db.insert_one(leaderboard).await;
        if let Err(e) = result {
            eprintln!("Error inserting leaderboard: {}", e);
        }
    }

    pub async fn update_leaderboard(&self, leaderboard: &Leaderboard) {
        let result = self
            .db
            .replace_one(
                doc! {
                    "_id": leaderboard.get_id()
                },
                leaderboard,
            )
            .await;

        if let Err(e) = result {
            eprintln!("Error updating leaderboard: {}", e);
        }
    }

    pub async fn get_leaderboards(&self) -> Vec<Leaderboard> {
        let re = self.db.find(doc! {}).await;
        match re {
            Ok(cursor) => cursor.try_collect().await.unwrap_or_default(),
            Err(err) => {
                eprintln!("Failed to fetch leaderboards: {}", err);
                vec![]
            }
        }
    }

    pub async fn find_leaderboard_for_player(&self, player_id: &ObjectId) -> Option<Leaderboard> {
        let result = self
            .db
            .find_one(doc! {
                "players": {
                    "$elemMatch": {
                        "id": player_id
                    }
                }
            })
            .await;

        result.unwrap_or_else(|e| {
            eprintln!("Error finding leaderboard for player: {}", e);
            None
        })
    }
}
