use super::NodeClient;
use crate::prelude::*;

#[derive(thiserror::Error, Debug)]
pub enum AppError {
    #[error("Env error: {0}")]
    Env(#[source] anyhow::Error),
    #[error("Node client error: {0}")]
    NodeClient(#[source] anyhow::Error),
    #[error("Storage error: {0}")]
    Storage(#[source] anyhow::Error),
}

#[derive(Debug, Clone)]
pub struct App<T: Storage> {
    pub client: NodeClient,
    pub storage: Option<T>,
    pub config: Env,
}

impl<T> App<T>
where
    T: Storage,
{
    pub async fn setup() -> DynResult<Self> {
        Env::init().map_err(AppError::Env)?;
        let client = NodeClient::connect().await.map_err(AppError::NodeClient)?;
        let config = Env::get();

        Ok(Self {
            config,
            client,
            storage: None,
        })
    }
}

impl App<PostgresDB> {
    pub async fn init_storage(&mut self, config: PgConfig) -> DynResult<Self> {
        let storage = PostgresDB::init(config).await.map_err(AppError::Storage)?;
        self.storage = Some(storage);
        Ok(self.clone())
    }
}
