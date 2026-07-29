use std::{
    fs::File,
    net::{Ipv4Addr, SocketAddr},
    path::{PathBuf, absolute},
};

use clap::Parser;
use file_type::FileType;
use tiny_http::{Header, Response, Server};

use crate::error::Error;

mod error;

#[derive(Parser)]
/// A simple HTTP server for serving a directory
struct Args {
    /// The directory to serve
    directory: PathBuf,

    /// The port to serve at
    #[clap(short, long, default_value_t = 3000)]
    port: u16,
}

fn main() -> Result<(), Error> {
    let args = Args::parse();

    let addr = SocketAddr::new(Ipv4Addr::LOCALHOST.into(), args.port);
    let server = Server::http(addr)?;
    println!("Server started at http://localhost:{}", args.port);

    for request in server.incoming_requests() {
        let url = request.url();
        let url = absolute(url)?;
        let file_path = match url.strip_prefix("/") {
            Ok(p) => args.directory.join(p),
            Err(_) => args.directory.join(url),
        };
        let file_path = if file_path.starts_with(&args.directory) {
            file_path
        } else {
            continue;
        };

        if file_path.is_file() {
            let file_type = FileType::try_from_file(&file_path)?;
            request.respond(Response::from_file(File::open(file_path)?).with_header(
                Header::from_bytes("Content-Type", file_type.media_types()[0])?,
            ))?;
            continue;
        }
        if let file_path = file_path.with_extension("html")
            && file_path.is_file()
        {
            request.respond(
                Response::from_file(File::open(file_path)?)
                    .with_header(Header::from_bytes("Content-Type", "text/html")?),
            )?;
            continue;
        }
        if let file_path = file_path.join("index.html")
            && file_path.is_file()
        {
            request.respond(
                Response::from_file(File::open(file_path)?)
                    .with_header(Header::from_bytes("Content-Type", "text/html")?),
            )?;
            continue;
        }
        request.respond(Response::empty(404))?;
    }

    Ok(())
}
