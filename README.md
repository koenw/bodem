# Bodem

Bodem is a simple Gopher server in rust.

## Why?

I really like how radically simple the Gopher protocol is. This feature is even
more appealing to me today, now that even a 'simple' web page involves a ton of
moving parts (e.g. let's encrypt integration, checking for vulnerabilities in
your dependencies, etc.).

To give you an example of how simple the Gopher protocol really is, here is an
example of using *netcat* to download a file over Gopher:

```sh
echo music/never_gonna_give_you_up.mp4 | nc gopher.example.com 70 > rickroll.mp4
```

I would encourage everyone to read the Gopher [RFC
1436](https://tools.ietf.org/html/rfc1436) and/or [wikipedia
entry](https://en.wikipedia.org/wiki/Gopher_(protocol)), if only for an
interesting piece of history.

## Usage

*Bodem* has only two configuration options: the address to listen on and the
directory to serve. The default listen address is `127.0.0.1:7070` and the
default root directory is the current directory.

Since there are only two configuration options, the following examples should
cover your use case.

All configuration can also be done through environment variables, so it's totally ready for the cloud :)

### Examples

| Command | Description |
|---------|-------------|
|`bodem -l 0.0.0.0:70` | Serve the current directory on port 70 (the official gopher port) on all interfaces. |
|`bodem /srv/gopher` | Serve the directory `/srv/gopher` on the default address (`127.0.0.1:7070`). |
|`bodem -l 172.16.11.135:1234 /some/stuff` | Serve the directory `/some/stuff` on `172.16.11.135:1234` (This of course will only work if you have an interface with this IP address configured). |

## Building

To build *bodem*, you'll need Rust and Cargo installed, for which I'll refer to
the [official documentation](https://www.rust-lang.org/tools/install), but
don't forget to check your local package manager.

| Command | Description |
|---------|-------------|
|`cargo build --release` | Build a non-debug binary to `target/release/bodem`. |
|`cargo run` | Run a debug build directly. |
|`cargo run --release` | Run a non-debug build directly. |
