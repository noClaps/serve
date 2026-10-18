# serve

A simple HTTP server for serving a directory.

## Usage

```
Usage: serve <directory>

A simple HTTP server for serving a directory.

Arguments:
  <directory>  The directory to serve.

Options:
  --html       Try the .html extension if the requested path doesn't exist.
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

For a given request `/path/to/url`, the order in which files are served are:

1. If `directory/path/to/url` exists, return it.
2. If the `--html` flag was passed in and `directory/path/to/url.html` exists, return it.
3. If `directory/path/to/url/index.html` exists, return it.
4. If `directory/404.html` exists, return it with status code 404.
5. Return an empty response with status code 404.

## Installation

You can build it from source using [Go](https://go.dev):

```sh
go install codeberg.org/noclaps/serve@latest
```
