use rmcp::serve_server;
use tracing_subscriber::{
    layer::SubscriberExt,
    util::SubscriberInitExt,
    {self},
};
mod counter;
use counter::Counter;
mod generic_service;
use generic_service::{GenericService, MemoryDataService};

const BIND_ADDRESS: &str = "127.0.0.1:8000";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "debug".to_string().into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let memory_service = MemoryDataService::new("initial data");

    let generic_service = GenericService::new(memory_service);

    println!("start server, connect to standard input/output");

    let io = (tokio::io::stdin(), tokio::io::stdout());

    serve_server(generic_service, io).await?.waiting().await?;
    Ok(())
}
