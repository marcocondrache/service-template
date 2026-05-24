mod app;

use std::net::SocketAddr;

use clap::Parser;

#[derive(Debug, Parser)]
#[command(author, version, about)]
struct Args {
    #[arg(long, env = "BIND", default_value = "0.0.0.0:8080")]
    bind: SocketAddr,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::try_parse()?;
    let listener = tokio::net::TcpListener::bind(args.bind).await?;

    println!("listening on http://{}", args.bind);

    axum::serve(listener, app::router()).await?;

    Ok(())
}
