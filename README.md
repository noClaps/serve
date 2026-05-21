# serve

A simple HTTP server for serving a directory.

## Installation

You can build it from source using [Rust](https://rust-lang.org):

```sh
cargo install --git https://github.com/noClaps/serve
```

## Usage

```
USAGE: serve <directory> [--port <port>]

POSITIONALS:
  <directory>          The directory to serve

FLAGS:
  -p, --port <port>    The port to serve at (default: 3000)
  -h, --help           Display this help and exit.
```

You can use the tool simply by running:

```sh
serve dist # or whatever path to the directory you want to serve
```

This will serve the `dist/` directory at `http://localhost:3000`.

You can also customise the port it serves at using the `--port` option:

```sh
serve static --port 4321
```

This will serve the `static/` directory at `http://localhost:4321`.

You can view the help by using `--help` or `-h`:

```sh
serve -h
serve --help
```
