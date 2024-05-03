use crate::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BlockEntity(Block);

impl Entity for BlockEntity {
    type ID = HexString;
    type Props = Block;
    fn id(&self) -> &Self::ID {
        &self.0.id
    }
    fn props(&self) -> &Self::Props {
        &self.0
    }
}

impl BlockEntity {
    pub fn new(input: Block) -> Self {
        Self(input)
    }
}
