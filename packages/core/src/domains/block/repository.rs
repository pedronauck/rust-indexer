use async_trait::async_trait;
use sqlx::QueryBuilder;

use super::{BlockEntity, BlockRecord, BlockRecordList};
use crate::prelude::*;

#[derive(thiserror::Error, Debug)]
pub enum BlockRepositoryError {
    #[error("Database connection error: {0}")]
    DatabaseError(String),
    #[error("Block not found with id: {0}")]
    NotFoundById(i32),
    #[error("Error selecting all blocks: {0}")]
    FindAllError(String),
    #[error("Error insert one block: {0}")]
    InsertOneError(String),
    #[error("Error inserting many blocks: {0}")]
    InsertManyError(String),
}

#[async_trait]
pub trait BlockRepositoryMethods {
    async fn find_many(&self) -> DynResult<Vec<BlockEntity>>;
    async fn insert_many(&self, blocks: Vec<ClientBlock>) -> DynResult<Vec<BlockEntity>>;
}

#[derive(Debug, Clone)]
pub struct BlockRepository<T: Storage> {
    pub storage: T,
}

impl<T> BlockRepository<T>
where
    T: Storage,
{
    pub fn new(storage: T) -> Self {
        Self { storage }
    }
}

#[async_trait]
impl BlockRepositoryMethods for BlockRepository<PostgresDB> {
    async fn find_many(&self) -> DynResult<Vec<BlockEntity>> {
        let db = self.storage.client.to_owned().unwrap();
        let query = r#"
        SELECT * FROM blocks
        "#;

        let result = sqlx::query_as::<_, BlockRecord>(query)
            .fetch_all(&db)
            .await?;

        dbg!(&result);

        let record_list = BlockRecordList::from(result);
        let entities = record_list.as_entities();
        Ok(entities)
    }
    async fn insert_many(&self, blocks: Vec<ClientBlock>) -> DynResult<Vec<BlockEntity>> {
        let db = self.storage.client.to_owned().unwrap();
        let list = BlockRecordList::from(blocks);
        let entities = list.as_entities();
        let mut query_builder = QueryBuilder::new("INSERT INTO blocks (id, data)");

        query_builder.push_values(list.records(), |mut q, record| {
            q.push_bind(record.id).push_bind(record.data);
        });

        let query = query_builder.build();
        let result = query.execute(&db).await?;

        dbg!(&result);
        Ok(entities)
    }
}
