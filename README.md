# rust-redis

A Redis-compatible server implementation in Rust. Listens on port 6379, speaks the [RESP protocol](https://redis.io/docs/reference/protocol-spec/), and runs async I/O on tokio.

## What it does

- **TCP server** — Binds to `127.0.0.1:6379` (Redis default)
- **RESP parser** — Handles the Redis Serialization Protocol (arrays, bulk strings)
- **In-memory storage** — HashMap-backed key-value store, shared across connections
- **Commands** — `PING`, `ECHO <msg>`, `SET key value`, `GET key`

## Quick start

```bash
cargo run
```

Then in another terminal, hit it with the Redis CLI:

```bash
redis-cli -p 6379 ping
# PONG

redis-cli -p 6379 echo "hello world"
# "hello world"

redis-cli -p 6379 set foo bar
# OK

redis-cli -p 6379 get foo
# "bar"
```

Or use `nc` / telnet and type raw RESP:

```
*1\r\n$4\r\nPING\r\n
```

## Running tests

```bash
cargo test
```

## Project layout

```
src/
├── main.rs           # Server entrypoint, connection handling
├── lib.rs            # Crate root, exports for tests
├── resp.rs           # RESP protocol parsing (arrays, bulk strings)
├── resp_results.rs   # RESP error types
├── storage.rs        # Command execution (PING, ECHO, SET, GET)
├── storage_result.rs # Storage error types
├── error.rs          # Unified error type
├── middleware.rs     # Placeholder for future middleware
└── commands/
    └── strings.rs    # String command implementations

tests/
└── integration_test.rs
```

## Dependencies

- **tokio** — async runtime (TCP, spawn)

## License

MIT
