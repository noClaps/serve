# serve

A simple HTTP server for serving a directory.

## Usage

```
usage: serve [-h] directory

a simple HTTP server for serving a directory

positional arguments:
  directory   the directory to serve

optional arguments:
  -h, --help  show this help message and exit
```

You can use the tool simply by running:

```sh
serve dist    # or whatever path to the directory you want to serve
```

This will serve the `dist/` directory at a random free port on your system.

You can view the help by using `--help` or `-h`:

```sh
serve -h
serve --help
```

For a given request `/path/to/url`, the order in which files are served are:

1. If `directory/path/to/url` exists, return it.
2. If `directory/path/to/url.html` exists, return it.
3. If `directory/path/to/url/index.html` exists, return it.
4. If `directory/404.html` exists, return it with status code 404.
5. Return an empty response with status code 404.

## Installation

You'll need [Python](https://python.org) (tested on Python 3.9.6 and 3.15.1). It should be built-in on most Linux and macOS systems

```sh
go install codeberg.org/noclaps/serve@latest
```
