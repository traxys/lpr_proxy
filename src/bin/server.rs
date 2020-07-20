use lpr_proxy::LprOptions;
use tempfile::tempdir;
use tokio::process::Command;
use tokio::stream::StreamExt;
use warp::{filters::multipart, http, reject, Buf, Filter};

#[derive(Debug)]
enum Error {
    Serde(serde_json::Error),
    ReadPart(warp::Error),
    NoDataInPart,
    InvalidPartName(String),
    WriteFile(std::io::Error),
    CreateTempDir(std::io::Error),
}
impl warp::reject::Reject for Error {}

async fn handle_form(mut form: multipart::FormData) -> Result<impl warp::Reply, warp::Rejection> {
    let mut options: Option<LprOptions> = None;
    let mut files = Vec::new();

    eprintln!("Recieved print job");

    let dir = tempdir()
        .map_err(Error::CreateTempDir)
        .map_err(reject::custom)?;

    while let Some(part) = form.next().await {
        let mut part = part.map_err(|err| reject::custom(Error::ReadPart(err)))?;
        let name = part.name();
        if name.starts_with("opt:") {
            options = Some(
                serde_json::from_slice(
                    part.data()
                        .await
                        .ok_or(reject::custom(Error::NoDataInPart))?
                        .map_err(Error::ReadPart)
                        .map_err(reject::custom)?
                        .bytes(),
                )
                .map_err(Error::Serde)
                .map_err(reject::custom)?,
            );
        } else if name.starts_with("file:") {
            let file_name = dir.path().join(name.trim_start_matches("file:").to_owned());

            let data = part
                .data()
                .await
                .ok_or(reject::custom(Error::NoDataInPart))?
                .map_err(Error::ReadPart)
                .map_err(reject::custom)?;

            tokio::fs::write(&file_name, data.bytes())
                .await
                .map_err(Error::WriteFile)
                .map_err(reject::custom)?;

            files.push(file_name);
        } else {
            return Err(warp::reject::custom(Error::InvalidPartName(
                name.to_owned(),
            )));
        }
    }

    let options = options.map(|o| o.to_options()).unwrap_or_else(Vec::new);
    let options = options.iter().map(|cow| -> &str { &*cow });

    eprintln!("Printing files: {:?}", files);
    let mut command = Command::new("lpr");
    command.args(options);
    command.args(files.into_iter());
    let status = command
        .spawn()
        .expect("could not run command")
        .wait_with_output()
        .await
        .expect("error running command");
    eprintln!("Finished with status: {:?}", status);


    Ok(warp::reply::with_status(
        "added to lpr",
        http::StatusCode::OK,
    ))
}

#[tokio::main]
async fn main() {
    let lpr_proxy = multipart::form().and_then(handle_form);

    warp::serve(lpr_proxy)
        .run(([127, 0, 0, 1], lpr_proxy::PORT))
        .await
}
