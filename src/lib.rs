use crate::db::Db;

pub mod db;
pub mod handler;
pub mod middleware;
pub mod model;

#[derive(Clone)]
pub struct App {
    pub db: Db,
}

impl App {
    pub async fn new() -> mongodb::error::Result<Self> {
        Ok(Self {
            db: Db::new().await?,
        })
    }
}
