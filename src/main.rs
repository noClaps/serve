use std::{
    fs::File,
    net::{Ipv4Addr, SocketAddr},
    path::{PathBuf, absolute},
};

use clap::Parser;
use tiny_http::{Response, Server};

#[derive(Parser)]
struct Args {
    #[clap(short, long, default_value_t = 3000)]
    /// The port to serve at
    port: u16,

    /// The directory to serve
    directory: PathBuf,
}

fn main() {
    let args = Args::parse();

    let addr = SocketAddr::new(Ipv4Addr::LOCALHOST.into(), args.port);
    let server = Server::http(addr).expect("Error creating server");
    println!("Server started at http://localhost:{}", args.port);

    for request in server.incoming_requests() {
        let url = request.url();
        let url = absolute(url).expect("Error converting URL");
        let file_path = match url.strip_prefix("/") {
            Ok(p) => args.directory.join(p),
            Err(_) => args.directory.join(url),
        };
        let file_path = if file_path.starts_with(&args.directory) {
            file_path
        } else {
            continue;
        };

        match true {
            true if file_path.is_file() => {
                request.respond(Response::from_file(File::open(file_path).unwrap()))
            }
            true if let file_path = file_path.with_extension("html")
                && file_path.is_file() =>
            {
                request.respond(Response::from_file(File::open(file_path).unwrap()))
            }
            true if let file_path = file_path.join("index.html")
                && file_path.is_file() =>
            {
                request.respond(Response::from_file(File::open(file_path).unwrap()))
            }
            _ => request.respond(Response::empty(404)),
        }
        .expect("Error sending response");
    }
}
