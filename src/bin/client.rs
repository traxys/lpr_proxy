use anyhow::Context;
use lpr_proxy::{LprOptions, PORT};
use reqwest::Client;
use serde::Serialize;
use structopt::StructOpt;

#[derive(StructOpt, Debug, Serialize)]
pub struct LprArgs {
    #[structopt(flatten)]
    options: LprOptions,
    #[structopt(long = "remote")]
    remote: String,
    #[structopt(long = "prefix")]
    prefix: Option<String>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut args = LprArgs::from_args();
    if let Some(prefix) = &args.prefix {
        args.options.truncate(prefix)?;
    }

    let client = Client::new();
    client
        .post(&format!("http://{}:{}", args.remote, PORT))
        .json(&args.options)
        .send()
        .await
        .with_context(|| "could not send request to server")?
        .error_for_status()
        .with_context(|| "request failed")?;

    eprintln!("Sent print request");

    Ok(())
}
