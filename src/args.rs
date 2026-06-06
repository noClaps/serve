use std::{env, path::PathBuf, process::exit};

pub struct Args {
    pub port: u16,
    pub directory: PathBuf,
}

impl Args {
    pub fn parse() -> Self {
        let mut args: Vec<_> = env::args().skip(1).collect();
        let mut parsed = Self {
            port: 3000,
            directory: PathBuf::new(),
        };

        if args.contains(&"-h".to_string()) || args.contains(&"--help".to_string()) {
            println!(
                r"
A simple HTTP server for serving a directory.

Usage: serve <directory> [--port <port>]

Arguments:
  <directory>          The directory to serve.

Options:
  -p, --port <port>    The port to serve at. (default: 3000)
  -h, --help           Show this help and exit.

You can use this tool simply by running:

  $ serve dist    # or whatever path to the directory you want to serve

This will serve the `dist/` directory at `http://localhost:3000`.

You can also customise the port using the `--port` or `-p` option:

  $ serve static --port 4321
  $ serve static -p 4321

This will serve the `static/` directory at `http://localhost:4321`.
"
            );
            exit(0);
        }

        if let Some(pos) = args.iter().position(|a| a == "-p" || a == "--port") {
            let Some(p) = args.get(pos + 1).cloned() else {
                println!(
                    r"
Missing optional parameter: <port>

Usage: serve <directory> [--port <port>]

See `serve --help` for the full documentation.
"
                );
                exit(1);
            };
            args.remove(pos + 1); // removes --port [port]
            args.remove(pos); // removes [--port]

            parsed.port = p
                .parse()
                .unwrap_or_else(|err| panic!("Error parsing `{p}` to u16: {err}"));
        }

        if args.is_empty() {
            println!(
                r"
Missing required argument: <directory>

Usage: serve <directory> [--port <port>]

See `serve --help` for the full documentation.
"
            );
            exit(1)
        }
        if args.len() > 1 {
            println!(
                r"
Extra argument provided: {}

Usage: serve <directory> [--port <port>]

See `serve --help` for the full documentation.
",
                args[1]
            );
            exit(1);
        }

        parsed.directory = PathBuf::from(args[0].clone());
        parsed
    }
}
