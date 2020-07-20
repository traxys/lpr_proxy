use lpr_proxy::LprOptions;
use std::sync::Arc;
use structopt::StructOpt;
use tokio::process::Command;
use warp::{filters::body, http, reject, Filter};

#[derive(Debug)]
enum Error {
    RunError(tokio::io::Error),
}
impl warp::reject::Reject for Error {}

async fn handle_options(
    mut options: LprOptions,
    args: Arc<ServerArgs>,
) -> Result<impl warp::Reply, warp::Rejection> {
    if let Some(prefix) = &args.prefix {
        options.rebuild(prefix);
    }
    eprintln!("Print job: {:?}", options);

    let options = options.to_options();

    let mut command = Command::new("lpr");
    command.args(options);
    let status = command
        .status()
        .await
        .map_err(Error::RunError)
        .map_err(reject::custom)?;
    eprintln!("Finished with status: {:?}", status);

    Ok(warp::reply::with_status(
        "added to lpr",
        http::StatusCode::OK,
    ))
}

#[derive(StructOpt)]
struct ServerArgs {
    prefix: Option<String>,
}

#[tokio::main]
async fn main() {
    let args = Arc::new(ServerArgs::from_args());
    let args = warp::any().map(move || args.clone());

    let lpr_proxy = body::content_length_limit(1024 * 4)
        .and(body::json())
        .and(args)
        .and_then(handle_options);

    warp::serve(lpr_proxy)
        .run(([127u8, 0, 0, 1], lpr_proxy::PORT))
        .await
}
