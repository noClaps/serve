# serve

A simple HTTP server for serving a directory.

## Usage

```
Usage: serve <directory>

A simple HTTP server for serving a directory.

Arguments:
  <directory>  The directory to serve.

Options:
  -h, --help   Show this help and exit.
```

You can use the tool simply by running:

```sh
serve dist    # or whatever path to the directory you want to serve
```

This will serve the `dist/` directory at `http://localhost:3000`. If port `3000` is occupied, it will try port `3001`, then `3002`, and so on.

You can view the help by using `--help` or `-h`:

```sh
serve -h
serve --help
```

## Installation

You can build it from source using [Rust](https://rust-lang.org):

```sh
cargo install --git https://codeberg.org/noClaps/serve
```
