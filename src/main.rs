use std::{
    fs::File,
    net::{Ipv4Addr, SocketAddr},
    path::{PathBuf, absolute},
    str::FromStr,
};

use argp::FromArgs;
use mime_guess::{Mime, MimeGuess};
use tiny_http_fork::{Header, Response, Server};

#[derive(FromArgs)]
/// A simple HTTP server for serving a directory.
struct Args {
    /// The directory to serve.
    #[argp(positional)]
    directory: PathBuf,

    /// The port to serve at. (default: 3000)
    #[argp(option, short = 'p', default = "3000")]
    port: u16,
}

fn main() {
    let args: Args = argp::parse_args_or_exit(argp::DEFAULT);

    let addr = SocketAddr::new(Ipv4Addr::LOCALHOST.into(), args.port);
    let server = Server::http(addr).unwrap();
    println!("Server started at http://localhost:{}", args.port);

    for request in server.incoming_requests() {
        let request = match request {
            Ok(r) => r,
            Err(_) => continue,
        };

        let url = absolute(request.url()).unwrap();
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
            let file_type =
                MimeGuess::from_path(&file_path).first_or(Mime::from_str("text/plain").unwrap());
            request
                .respond(
                    Response::from_file(File::open(file_path).unwrap())
                        .with_header(Header::from_str("Content-Type", &file_type).unwrap()),
                )
                .unwrap();
            continue;
        }
        if let file_path = file_path.with_extension("html")
            && file_path.is_file()
        {
            request
                .respond(
                    Response::from_file(File::open(file_path).unwrap())
                        .with_header(Header::from_str("Content-Type", "text/html").unwrap()),
                )
                .unwrap();
            continue;
        }
        if let file_path = file_path.join("index.html")
            && file_path.is_file()
        {
            request
                .respond(
                    Response::from_file(File::open(file_path).unwrap())
                        .with_header(Header::from_str("Content-Type", "text/html").unwrap()),
                )
                .unwrap();
            continue;
        }
        request.respond(Response::empty(404)).unwrap();
    }
}
