use crate::domains::block::*;
use crate::prelude::*;
use async_trait::async_trait;

#[derive(Debug, Clone)]
pub struct SyncAllBlocks<T: Storage> {
    client: NodeClient,
    per_page: i32,
    page_direction: PageDirection,
    storage: T,
}

impl<T> SyncAllBlocks<T>
where
    T: Storage + Clone,
{
    pub fn new(app: &App<T>) -> Self {
        Self {
            client: app.client.to_owned(),
            per_page: 100,
            page_direction: PageDirection::Forward,
            storage: app.storage.as_ref().unwrap().clone(),
        }
    }
    pub fn set_per_page(mut self, per_page: i32) -> Self {
        self.per_page = per_page;
        self
    }
    pub fn set_page_direction(mut self, page_direction: PageDirection) -> Self {
        self.page_direction = page_direction;
        self
    }
    pub async fn blocks_from_client(&self) -> DynResult<Vec<ClientBlock>> {
        let client = self.client.get();
        let data = client
            .get_blocks(PaginationRequest {
                cursor: None,
                results: self.per_page,
                direction: self.page_direction,
            })
            .await?;

        let blocks: Vec<ClientBlock> = data.results;
        Ok(blocks)
    }
}

pub type SyncAllBlocksResponse = DynResult<Vec<BlockEntity>>;

#[async_trait]
impl UseCase for SyncAllBlocks<PostgresDB> {
    type Response = SyncAllBlocksResponse;
    async fn execute(&self) -> Self::Response {
        let blocks = self.blocks_from_client().await?;
        let repository = BlockRepository::new(self.storage.clone());
        let result = repository.insert_many(blocks).await?;
        Ok(result)
    }
}
