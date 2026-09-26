use std::{
    env::args,
    fs::File,
    net::{Ipv4Addr, SocketAddr},
    path::{PathBuf, absolute},
    process::exit,
    str::FromStr,
};

use mime_guess::{Mime, MimeGuess};
use tiny_http_fork::{Header, Response, Server};

fn main() {
    let args: Vec<_> = args().skip(1).collect();
    if args.contains(&"-h".to_string()) || args.contains(&"--help".to_string()) {
        println!(
            "\
Usage: serve <directory>

A simple HTTP server for serving a directory.

Arguments:
  <directory>  The directory to serve.

Options:
  -h, --help   Show this help and exit."
        );
        return;
    }
    if args.is_empty() {
        eprintln!("Missing required argument: <directory>");
        exit(1);
    }

    let directory = PathBuf::from(&args[0]);
    const PORT: u16 = 3000;

    let addr = SocketAddr::new(Ipv4Addr::LOCALHOST.into(), PORT);
    let server = Server::http(addr).unwrap();
    println!("Server started at http://localhost:{PORT}");

    for request in server.incoming_requests() {
        let request = match request {
            Ok(r) => r,
            Err(_) => continue,
        };

        let url = absolute(request.url()).unwrap();
        let file_path = match url.strip_prefix("/") {
            Ok(p) => directory.join(p),
            Err(_) => directory.join(url),
        };
        let file_path = if file_path.starts_with(&directory) {
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
