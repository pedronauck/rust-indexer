use crate::prelude::*;

#[derive(thiserror::Error, Debug)]
pub enum NodeClientError {
    #[error("Client error: {0}")]
    ClientError(#[from] fuels::prelude::Error),
}

#[derive(Debug, Clone)]
pub struct NodeClient(Provider);

impl NodeClient {
    pub async fn connect() -> DynResult<Self> {
        let env = Env::get();
        let client = Provider::connect(&env.node_url)
            .await
            .map_err(NodeClientError::ClientError)?;

        Ok(Self(client))
    }
    pub fn get(&self) -> &Provider {
        &self.0
    }
}
