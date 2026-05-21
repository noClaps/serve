use std::{
    net::{Ipv4Addr, SocketAddr},
    path::PathBuf,
};

use argparse::ArgParse;
use axum::{
    Router,
    extract::State,
    http::{StatusCode, Uri},
    response::{IntoResponse, Response},
};
use tokio::{io, net::TcpListener};
use tower_http::services::ServeDir;

#[derive(Clone)]
struct AppState {
    root: PathBuf,
}

#[tokio::main]
async fn main() -> Result<(), io::Error> {
    let args = ArgParse::new()
        .positional("directory", "The directory to serve")
        .flag("port", "port", Some("p"), "The port to serve at", false)
        .parse();
    let port = args.flag("port").unwrap();
    let directory = args.positional("directory").unwrap();

    let addr = SocketAddr::new(Ipv4Addr::LOCALHOST.into(), port);
    let listener = TcpListener::bind(addr).await?;
    let router = Router::new().fallback_service(
        ServeDir::new(&directory).fallback(
            Router::new()
                .fallback(fallback)
                .with_state(AppState { root: directory }),
        ),
    );

    println!("Server started at http://localhost:{port}");
    axum::serve(listener, router).await
}

async fn fallback(State(state): State<AppState>, uri: Uri) -> Response {
    let path = state
        .root
        .join(format!("{}.html", uri.path().trim_start_matches("/")));

    match tokio::fs::read_to_string(path).await {
        Ok(contents) => (StatusCode::OK, [("Content-Type", "text/html")], contents).into_response(),
        Err(_) => StatusCode::NOT_FOUND.into_response(),
    }
}
