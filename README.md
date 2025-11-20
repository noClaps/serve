# serve

A simple HTTP server for serving a directory.

## Installation

You can build it from source using [Go](https://go.dev):

```sh
go install github.com/noclaps/serve@latest
```

## Usage

```
Usage of serve:
  -host string
    	The hostname to serve at. (default "dir.localhost")
  -port uint
    	The port to serve at. (default 3000)
```

You can use the tool simply by running:

```sh
serve dist # or whatever path to the directory you want to serve
```

You can also customise the port it serves at using the `-port` option:

```sh
serve static -port 4321
```

You can customise the hostname it serves at using the `-host` option:

```sh
serve static -host 'project.localhost'
```

The default hostname is the name of the directory you're running `serve` from, with `.localhost`. For example, if you run it from a directory called `myproject/`, the default hostname will be `myproject.localhost`.

You can view the help by using `-help` or `-h`:

```sh
serve -h
serve -help
```
