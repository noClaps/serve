use std::{
    net::{Ipv4Addr, SocketAddr},
    path::PathBuf,
};

use axum::{
    Router,
    extract::State,
    http::{StatusCode, Uri},
    response::{IntoResponse, Response},
};
use clap::Parser;
use tokio::net::TcpListener;
use tower_http::services::ServeDir;

#[derive(Parser)]
struct Args {
    #[clap(short, long, default_value_t = 3000)]
    /// The port to serve at
    port: u16,

    /// The directory to serve
    directory: PathBuf,
}

#[derive(Clone)]
struct AppState {
    root: PathBuf,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let addr = SocketAddr::new(Ipv4Addr::LOCALHOST.into(), args.port);
    let listener = TcpListener::bind(addr)
        .await
        .expect("Error while creating listener");
    let router = Router::new().fallback_service(ServeDir::new(&args.directory).fallback(
        Router::new().fallback(fallback).with_state(AppState {
            root: args.directory,
        }),
    ));

    println!("Server started at http://localhost:{}", args.port);
    axum::serve(listener, router)
        .await
        .expect("Error while running server");
}

async fn fallback(State(state): State<AppState>, uri: Uri) -> Response {
    let path = state
        .root
        .join(format!("{}.html", uri.path().trim_start_matches('/')));

    match tokio::fs::read_to_string(path).await {
        Ok(contents) => (StatusCode::OK, [("Content-Type", "text/html")], contents).into_response(),
        Err(_) => StatusCode::NOT_FOUND.into_response(),
    }
}
