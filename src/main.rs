use taos::{AsyncTBuilder, TaosBuilder};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let _taos = TaosBuilder::from_dsn("ws://localhost:6041")?
        .build()
        .await?;

    Ok(())
}
