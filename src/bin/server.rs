use lpr_proxy::{LprOptions, PORT_STR};
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
    #[structopt(long, default_value = "127.0.0.1")]
    listen: std::net::IpAddr,
    #[structopt(long, default_value = PORT_STR)]
    port: u16,
    #[structopt(long)]
    prefix: Option<String>,
}

#[tokio::main]
async fn main() {
    let args = Arc::new(ServerArgs::from_args());
    let args_f = args.clone();
    let args_filter = warp::any().map(move || args_f.clone());

    let lpr_proxy = body::content_length_limit(1024 * 4)
        .and(body::json())
        .and(args_filter)
        .and_then(handle_options);

    warp::serve(lpr_proxy).run((args.listen, args.port)).await
}
