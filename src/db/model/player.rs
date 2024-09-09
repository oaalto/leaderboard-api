use futures::stream::TryStreamExt;
use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;
use mongodb::Collection;

use crate::model::Player;

#[derive(Clone)]
pub struct PlayerModel {
    db: Collection<Player>,
}

impl PlayerModel {
    pub fn new(db: Collection<Player>) -> Self {
        Self { db }
    }

    pub async fn find_player(&self, id: &ObjectId) -> Option<Player> {
        let result = self
            .db
            .find_one(doc! {
                "_id": id
            })
            .await;

        match result {
            Err(e) => {
                eprintln!("Failed to fetch player {:?}: {}", id, e);
                None
            }
            Ok(player) => player,
        }
    }

    pub async fn add_player(&self, player: &Player) {
        let result = self.db.insert_one(player).await;
        if let Err(e) = result {
            eprintln!("Error inserting player: {}", e);
        }
    }

    pub async fn get_players(&self) -> Vec<Player> {
        let re = self.db.find(doc! {}).await;
        match re {
            Ok(cursor) => cursor.try_collect().await.unwrap_or_default(),
            Err(err) => {
                eprintln!("Failed to fetch players: {}", err);
                vec![]
            }
        }
    }
}
