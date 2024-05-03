use fdx_core::{cases::block::*, prelude::*};

pub struct Sync<T: Storage> {
    app: App<T>,
}

impl Sync<PostgresDB> {
    pub fn new(app: App<PostgresDB>) -> Self {
        Self { app }
    }
    pub async fn sync_all_blocks(&self) -> FindAllBlocksResponse {
        let sync_all_blocks = SyncAllBlocks::new(&self.app);
        let response = sync_all_blocks.execute().await?;
        Ok(response)
    }
    pub async fn find_all_blocks(&self) -> SyncAllBlocksResponse {
        let find_all_blocks = FindAllBlocks::new(&self.app);
        let response = find_all_blocks.execute().await?;
        Ok(response)
    }
}
