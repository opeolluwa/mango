use std::sync::Arc;

use sqlx::Postgres;
use sqlx::pool::Pool;

#[derive(Clone)]
pub struct PlaylistRepository {
    pool: Arc<Pool<Postgres>>,
}

impl PlaylistRepository {
    pub fn init(pool: &Pool<Postgres>) -> Self {
        Self {
            pool: Arc::new(pool.clone()),
        }
    }
}

pub trait PlaylistRepositoryExt {
    fn create_new() -> impl std::future::Future<Output = ()> + Send;
    fn update() -> impl std::future::Future<Output = ()> + Send;
    fn delete() -> impl std::future::Future<Output = ()> + Send;
    fn fetch_one() -> impl std::future::Future<Output = ()> + Send;
    fn fetch_many() -> impl std::future::Future<Output = ()> + Send;

}

impl PlaylistRepositoryExt for PlaylistRepository  {
    async fn create_new() {
        todo!()
    }

    async fn update() {
        todo!()
    }

    async fn delete() {
        todo!()
    }

    async fn fetch_one() {
        todo!()
    }

    async fn fetch_many() {
        todo!()
    }
}