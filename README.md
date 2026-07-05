# serve

A simple HTTP server for serving a directory.

## Usage

```
Usage: serve <DIRECTORY> [--port <port>]

Arguments:
  <DIRECTORY>          The directory to serve.

Options:
  -p, --port <port>    The port to serve at. (default: 3000)
  -h, --help           Display this help and exit.
  -V, --version        Display version and exit.
```

You can use the tool simply by running:

```sh
serve dist    # or whatever path to the directory you want to serve
```

This will serve the `dist/` directory at `http://localhost:3000`.

You can also customise the port it serves at using the `--port` or `-p` option:

```sh
serve static --port 4321
serve static -p 4321
```

This will serve the `static/` directory at `http://localhost:4321`.

You can view the help by using `--help` or `-h`:

```sh
serve -h
serve --help
```

## Installation

You can download a version for your system from [Releases](https://github.com/noClaps/serve/releases).

You can also build it from source using [Zig](https://ziglang.org):

```sh
git clone https://github.com/noClaps/serve.git && cd serve
zig build
```
