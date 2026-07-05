const std = @import("std");
const clap = @import("clap");
const config = @import("config");
const httpz = @import("httpz");

pub fn main(init: std.process.Init) !void {
    var stdout_buffer: [1024]u8 = undefined;
    var stdout_file_writer = std.Io.File.Writer.init(.stdout(), init.io, &stdout_buffer);
    const stdout = &stdout_file_writer.interface;

    const params = comptime clap.parseParamsComptime(
        \\-h, --help
        \\-V, --version
        \\-p, --port <u16>
        \\<str>
    );
    var diag = clap.Diagnostic{};
    var res = clap.parse(clap.Help, &params, clap.parsers.default, init.minimal.args, .{
        .allocator = init.gpa,
        .diagnostic = &diag,
    }) catch |err| {
        try diag.reportToFile(init.io, .stderr(), err);
        return err;
    };
    defer res.deinit();

    if (res.args.help != 0) {
        try stdout.print(
            \\
            \\A simple HTTP server for serving a directory.
            \\
            \\Usage: serve <DIRECTORY> [--port <port>]
            \\
            \\Arguments:
            \\  <DIRECTORY>          The directory to serve.
            \\
            \\Options:
            \\  -p, --port <port>    The port to serve at. (default: 3000)
            \\  -h, --help           Display this help and exit.
            \\  -V, --version        Display version and exit.
            \\
            \\You can use this tool simply by running:
            \\
            \\  $ serve dist    # or whatever path to the directory you want to serve
            \\
            \\This will serve the `dist/` directory at `http://localhost:3000`.
            \\
            \\You can also customise the port using the `--port` or `-p` option:
            \\
            \\  $ serve static --port 4321
            \\  $ serve static -p 4321
            \\
            \\This will serve the `static/` directory at `http://localhost:4321`.
            \\
            \\
        , .{});
        try stdout.flush();
        return;
    }
    if (res.args.version != 0) {
        try stdout.print("serve {s}\n", .{config.version});
        try stdout.flush();
        return;
    }
    if (res.positionals[0] == null) {
        std.log.err(
            \\Missing required argument: <DIRECTORY>
            \\Usage: cite <DIRECTORY>
            \\
            \\For more information, try '--help'.
        , .{});
        std.process.exit(1);
    }

    var handler = Handler{
        .root = res.positionals[0].?,
        .alloc = init.gpa,
        .io = init.io,
        .cwd = std.Io.Dir.cwd(),
    };
    var server = try httpz.Server(*Handler).init(
        init.io,
        init.gpa,
        .{ .address = .localhost(res.args.port orelse 3000) },
        &handler,
    );
    defer {
        server.stop();
        server.deinit();
    }
    var router = try server.router(.{});
    router.get("/*", Handler.handle, .{});
    std.log.info("Started HTTP server at http://localhost:{d}", .{res.args.port orelse 3000});
    try server.listen();
}

const Handler = struct {
    root: []const u8,
    alloc: std.mem.Allocator,
    io: std.Io,
    cwd: std.Io.Dir,

    fn handle(app: *Handler, req: *httpz.Request, res: *httpz.Response) !void {
        const writer = res.writer();

        var path = try std.Io.Dir.path.join(app.alloc, &[_][]const u8{ app.root, req.url.path });
        const metadata = app.cwd.statFile(app.io, path, .{}) catch blk: {
            path = try std.mem.concat(app.alloc, u8, &[_][]const u8{ path, ".html" });
            break :blk try app.cwd.statFile(app.io, path, .{});
        };
        switch (metadata.kind) {
            .file => {
                const buf = try app.cwd.readFileAlloc(app.io, path, app.alloc, .unlimited);
                res.content_type = .forFile(path);
                try writer.print("{s}", .{buf});
                try writer.flush();
            },
            .directory => {
                const index = try std.Io.Dir.path.join(app.alloc, &[_][]const u8{ path, "index.html" });
                const buf = try app.cwd.readFileAlloc(app.io, index, app.alloc, .unlimited);
                res.content_type = .HTML;
                try writer.print("{s}", .{buf});
                try writer.flush();
            },
            else => {
                res.setStatus(.not_found);
                try writer.print("Not found", .{});
                try writer.flush();
            },
        }
    }
};
