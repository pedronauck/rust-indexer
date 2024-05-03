use fdx_core::prelude::*;
use fdx_sync::Sync;

#[tokio::main]
async fn main() -> DynResult<()> {
    let mut app = App::<PostgresDB>::setup().await?;
    app.init_storage(PgConfig::new()).await?;

    let sync = Sync::<PostgresDB>::new(app);
    sync.sync_all_blocks().await.expect("Failed to run CLI");
    sync.find_all_blocks().await.expect("Failed to run CLI");

    Ok(())
}
