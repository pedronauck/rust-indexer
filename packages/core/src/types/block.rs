use bincode::{Decode, Encode};
use serde::{Deserialize, Serialize};

use super::HexString;

// --------------------------------------------------------------------------------
// Block
// --------------------------------------------------------------------------------

pub type ClientHeader = fuels::types::block::Header;
pub type ClientBlock = fuels::types::block::Block;

#[derive(Debug, Clone, Serialize, Deserialize, Encode, Decode)]
pub struct Header {
    pub id: HexString,
    pub da_height: u64,
    pub transactions_count: u64,
    pub message_receipt_count: u64,
    pub transactions_root: HexString,
    pub message_receipt_root: HexString,
    pub height: u32,
    pub prev_root: HexString,
    pub time: String,
    pub application_hash: HexString,
}
impl From<ClientHeader> for Header {
    fn from(client_header: ClientHeader) -> Self {
        Self {
            id: client_header.id.into(),
            da_height: client_header.da_height,
            transactions_count: client_header.transactions_count,
            message_receipt_count: client_header.message_receipt_count,
            transactions_root: client_header.transactions_root.into(),
            message_receipt_root: client_header.message_receipt_root.into(),
            height: client_header.height,
            prev_root: client_header.prev_root.into(),
            time: client_header.time.unwrap().to_string(),
            application_hash: client_header.application_hash.into(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Encode, Decode)]
pub struct Block {
    pub id: HexString,
    pub header: Header,
}
impl From<ClientBlock> for Block {
    fn from(client_block: ClientBlock) -> Self {
        Self {
            id: client_block.id.into(),
            header: client_block.header.into(),
        }
    }
}
