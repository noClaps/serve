# Changelog

# v0.1.0

The initial release of serve!

To use serve, you can simply run:

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
