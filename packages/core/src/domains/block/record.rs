use serde::{Deserialize, Serialize};

use super::BlockEntity;
use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct BlockRecord {
    pub id: i32,
    pub data: Vec<u8>,
}
impl BlockRecord {
    fn new(id: i32, data: Vec<u8>) -> Self {
        Self { id, data }
    }
    fn from_block(block: Block) -> Self {
        let id = block.header.height as i32;
        let config = bincode::config::standard();
        let data = bincode::encode_to_vec(block, config).unwrap();
        Self::new(id, data)
    }
    fn to_block(&self) -> DynResult<Block> {
        let config = bincode::config::standard();
        let (block, _): (Block, usize) =
            bincode::decode_from_slice(&self.data[..], config).unwrap();
        Ok(Block { ..block })
    }
}
impl From<ClientBlock> for BlockRecord {
    fn from(block: ClientBlock) -> Self {
        BlockRecord::from_block(Block::from(block))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockRecordList {
    records: Vec<BlockRecord>,
}

impl BlockRecordList {
    pub fn as_entities(&self) -> Vec<BlockEntity> {
        self.records
            .iter()
            .map(|record| record.to_block().unwrap())
            .collect::<Vec<Block>>()
            .into_iter()
            .map(BlockEntity::new)
            .collect()
    }
    pub fn records(&self) -> Vec<BlockRecord> {
        self.records.clone()
    }
}
impl From<Vec<BlockRecord>> for BlockRecordList {
    fn from(records: Vec<BlockRecord>) -> Self {
        Self { records }
    }
}
impl From<Vec<ClientBlock>> for BlockRecordList {
    fn from(blocks: Vec<ClientBlock>) -> Self {
        let records = blocks
            .into_iter()
            .map(|block| BlockRecord::from_block(Block::from(block.clone())))
            .collect::<Vec<BlockRecord>>();
        Self { records }
    }
}
