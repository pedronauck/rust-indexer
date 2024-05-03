use crate::types::DynResult;
use async_trait::async_trait;

#[async_trait]
pub trait Storage {
    type Client;
    type Config;

    async fn init(config: Self::Config) -> DynResult<Self>
    where
        Self: Sized + Clone;

    fn instance() -> Self;
    fn config() -> Self::Config;
    fn client(&self) -> DynResult<Self::Client>;
}
