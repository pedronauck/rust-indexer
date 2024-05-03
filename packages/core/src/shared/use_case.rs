use async_trait::async_trait;

#[async_trait]
pub trait UseCase {
    type Response;
    async fn execute(&self) -> Self::Response;
}
