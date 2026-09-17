#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let db = database::connect().await?;
    backend::run(db).await?;
    Ok(())
}