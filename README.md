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

*Bodem* only has a few configuration options, which you can see in the table
below. Basically, you tell it what directory to serve and what port to listen
on. Because the gopher protocol has no concept of relative links, *bodem* also
needs to know what address clients can connect to in order to construct working
links.

| short | long | env | default | description |
|---|---|---|---|---|
|`-l`|`--listen`|`BODEM_LISTEN`|127.0.0.1:7070|Address to listen on, in the form *<ip>:<port>*.|
|||`BODEM_ROOT`|`$PWD`|Directory to serve. |
||`--port`|`BODEM_EXTERNAL_PORT`|7070|External port that clients can connect to. Used to construct links.|
||`--host`|`BODEM_EXTERNAL_HOST`|localhost|External hostname that clients can connect to. Used to construct links. |
|`-h`|`--help`|||Print a help message and quit.|

### Examples

| Command | Description |
|---------|-------------|
|`bodem -l 0.0.0.0:70 --host gopher.example.com` | Serve the current directory on port 70 (the official gopher port) on all interfaces. |
|`bodem /srv/gopher` | Serve the directory `/srv/gopher` on the default address (`127.0.0.1:7070`). |
|`bodem -l 172.16.11.135:1234 --host gopher.internal.example.com --port 1234 /some/stuff` | Serve the directory `/some/stuff` on `172.16.11.135:1234` (This of course will only work if you have an interface with this IP address configured). |

## Building

To build *bodem*, you'll need Rust and Cargo installed, for which I'll refer to
the [official documentation](https://www.rust-lang.org/tools/install), but
don't forget to check your local package manager.

| Command | Description |
|---------|-------------|
|`cargo build --release` | Build a non-debug binary to `target/release/bodem`. |
|`cargo run` | Run a debug build directly. |
|`cargo run --release` | Run a non-debug build directly. |
