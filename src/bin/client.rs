use anyhow::Context;
use lpr_proxy::{LprOptions, PORT};
use reqwest::{multipart, Client};
use structopt::StructOpt;
use tokio::fs;

const URL: &str = "http://localhost";

#[derive(StructOpt, Debug)]
pub struct LprArgs {
    #[structopt(flatten)]
    options: LprOptions,
    files: Vec<std::path::PathBuf>,
}

impl LprArgs {
    async fn to_form(self) -> anyhow::Result<multipart::Form> {
        let mut form = multipart::Form::new();
        form = form.text(
            "opt:options",
            serde_json::to_string(&self.options).with_context(|| "could not write options")?,
        );

        for file_name in self.files {
            let file = fs::read(&file_name).await.with_context(|| {
                format!("could not read input file {}", &file_name.to_string_lossy())
            })?;
            let file_name = file_name.to_string_lossy().to_string();

            form = form.part(
                format!("file:{}", file_name),
                multipart::Part::bytes(file).file_name(file_name),
            )
        }

        Ok(form)
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = LprArgs::from_args();
    let form = args.to_form().await?;

    let client = Client::new();
    client
        .post(&format!("{}:{}", URL, PORT))
        .multipart(form)
        .send()
        .await
        .with_context(|| "could not send request to server")?
        .error_for_status()
        .with_context(|| "request failed")?;

    Ok(())
}