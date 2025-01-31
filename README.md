> [!WARNING]
> This project is still in development, and its main objective is for me to learn Rust.
> Please do not use this in production.

# chelaid - ID generator

`chelaid` = 🦀`chelate` + 🪪`id`

`chelaid` is reinmlementation of [katsubushi](https://github.com/kayac/go-katsubushi) in Rust.

## Usage

```console
$ ./chelaid -h
Usage: chelaid [OPTIONS] --worker-id <WORKER_ID>

Options:
  -w, --worker-id <WORKER_ID>  Worker ID
      --issue                  Issue a ID
      --issues <ISSUES>        Number of IDs to issue
      --host <HOST>            Host name for TCP server [default: 127.0.0.1]
      --port <PORT>            Port number for TCP server [default: 11212]
  -h, --help                   Print help
  -V, --version                Print versio
```

### Server

```console
$ CHELAID_LOG=debug ./target/debug/chelaid --worker-id 123 --port 11212
[2025-01-31T21:52:56Z DEBUG chelaid] start to run TCP server...
[2025-01-31T21:52:56Z INFO  chelaid::infra::server::tcp] start TCP server
[2025-01-31T21:52:56Z DEBUG chelaid::infra::server::tcp::worker] worker 0 started
[2025-01-31T21:52:56Z DEBUG chelaid::infra::server::tcp::worker] worker 0 waiting message
[2025-01-31T21:52:56Z DEBUG chelaid::infra::server::tcp::worker] worker 1 started
[2025-01-31T21:52:56Z DEBUG chelaid::infra::server::tcp::worker] worker 2 started
[2025-01-31T21:52:56Z DEBUG chelaid::infra::server::tcp::worker] worker 3 started

(connection from client)

[2025-01-31T21:52:59Z DEBUG chelaid::infra::server::tcp] accepted connection from 127.0.0.1:61792
[2025-01-31T21:52:59Z DEBUG chelaid::infra::server::tcp::pool] sent a job
[2025-01-31T21:52:59Z DEBUG chelaid::infra::server::tcp::worker] worker 0 got a job
[2025-01-31T21:52:59Z DEBUG chelaid::infra::server::tcp::worker] worker 1 waiting message
[2025-01-31T21:52:59Z DEBUG chelaid::infra::server::tcp] start waiting message loop from 127.0.0.1:61792
[2025-01-31T21:52:59Z DEBUG chelaid::infra::server::tcp] waiting message from 127.0.0.1:61792
[2025-01-31T21:53:02Z DEBUG chelaid::infra::server::tcp] response for 127.0.0.1:61792: VALUE foo 0 1335004945229262848
[2025-01-31T21:53:02Z DEBUG chelaid::infra::server::tcp] waiting message from 127.0.0.1:61792

(quit from client)

[2025-01-31T21:53:05Z DEBUG chelaid::infra::server::tcp] QUIT command from 127.0.0.1:61792
[2025-01-31T21:53:05Z DEBUG chelaid::infra::server::tcp] connection closed for 127.0.0.1:61792
[2025-01-31T21:53:05Z DEBUG chelaid::infra::server::tcp::worker] worker 0 started
```

### Client

```console
❯ telnet 127.0.0.1 11212
Trying 127.0.0.1...
Connected to 127.0.0.1.
Escape character is '^]'.
get foo
VALUE foo 0 1335004945229262848
quit
Connection closed by foreign host.
```

## Development

Setup development environment for Rust, then:

```console
$ cargo test
```

### Features

- Protocols
    - [ ] [Memcached Basic Text Protocol](https://docs.memcached.org/protocols/basic/)
        - [x] `get`
        - [ ] `stat`
        - [ ] `version`
        - [x] `quit`
    - [ ] [Memcached Meta Text Protocol](https://docs.memcached.org/protocols/meta/)
    - [ ] [Memcached Binary Protocol](https://docs.memcached.org/protocols/binary/)
        - ⚠️ [Deprecated](https://docs.memcached.org/protocols/#why-is-the-binary-protocol-deprecated) in Memcached
    - [ ] [HTTP Protocol]
        - [ ] `GET /id`
        - [ ] `GET /ids?n={num}`
        - [ ] `GET /stats`
    - [ ] [gRPC Protocol]
- Others
    - [ ] Issue worker ID using Redis

## LICENSE

[MIT](./LICENSE)

## AUTHOR

[handlename](https://github.com/handlename)
