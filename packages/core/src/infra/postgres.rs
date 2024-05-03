use crate::prelude::*;

use anyhow::anyhow;
use async_trait::async_trait;
use once_cell::sync::Lazy;
use sqlx::{Pool, Postgres};
use std::sync::Mutex;

pub type PgClient = Option<Pool<Postgres>>;
pub type PgConfig = sqlx::postgres::PgConnectOptions;

static INSTANCE: Lazy<Mutex<PostgresDB>> = Lazy::new(|| Mutex::new(PostgresDB::default()));

#[derive(Default, Debug, Clone)]
pub struct PostgresDB {
    pub client: PgClient,
    pub config: PgConfig,
}

#[async_trait]
impl Storage for PostgresDB {
    type Client = PgClient;
    type Config = PgConfig;

    async fn init(config: Self::Config) -> DynResult<Self> {
        let pool = sqlx::postgres::PgPoolOptions::new()
            .max_connections(5)
            .connect_with(config.to_owned())
            .await?;

        let mut instance = INSTANCE.lock().unwrap();
        if instance.client.is_some() {
            return Err(anyhow!("PostgresDB already initialized"));
        }

        let item = Self {
            client: Some(pool),
            config,
        };

        *instance = item.to_owned();
        Ok(item)
    }

    fn instance() -> Self {
        INSTANCE.lock().unwrap().clone()
    }

    fn config() -> Self::Config {
        INSTANCE.lock().unwrap().config.clone()
    }

    fn client(&self) -> DynResult<Self::Client> {
        let conn = self.client.clone();
        if conn.is_none() {
            return Err(anyhow!("PostgresDB not initialized"));
        }
        Ok(conn)
    }
}
