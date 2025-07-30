# serve

A simple HTTP server for serving a directory.

## Installation

You can build it from source using [Go](https://go.dev):

```sh
go install github.com/noclaps/serve@latest
```

or download one of the prebuilt binaries in [Releases](https://github.com/noClaps/serve/releases).

## Usage

```
USAGE: serve <directory> [--port <port>]

ARGUMENTS:
  <directory>              The directory to serve.

OPTIONS:
  -p, --port <port>        The port to serve at. (default: 3000)
  -h, --help               Display this help and exit.
```

You can use the tool simply by running:

```sh
serve dist # or whatever path to the directory you want to serve
```

You can also customise the port it serves at using the `--port` or `-p` option:

```sh
serve static -p 4321
serve static --port 4321
```

You can view the help by using `--help` or `-h`:

```sh
serve -h
serve --help
```
