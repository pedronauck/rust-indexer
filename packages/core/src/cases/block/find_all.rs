use crate::domains::block::*;
use crate::prelude::*;
use async_trait::async_trait;

#[derive(Debug, Clone)]
pub struct FindAllBlocks<T: Storage> {
    storage: T,
}
impl<T> FindAllBlocks<T>
where
    T: Storage + Clone,
{
    pub fn new(app: &App<T>) -> Self {
        Self {
            storage: app.storage.as_ref().unwrap().clone(),
        }
    }
}

pub type FindAllBlocksResponse = DynResult<Vec<BlockEntity>>;

#[async_trait]
impl UseCase for FindAllBlocks<PostgresDB> {
    type Response = FindAllBlocksResponse;

    async fn execute(&self) -> Self::Response {
        let repository = BlockRepository::new(self.storage.clone());
        let blocks = repository.find_many().await?;
        Ok(blocks)
    }
}
