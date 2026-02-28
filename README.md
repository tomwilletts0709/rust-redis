# rust-redis

A Redis-compatible server implementation in Rust. Listens on port 6379, speaks the [RESP protocol](https://redis.io/docs/reference/protocol-spec/), and runs async I/O on tokio.

## What it does

- **TCP server** — Binds to `127.0.0.1:6379` (Redis default)
- **RESP parser** — Handles the Redis Serialization Protocol
- **In-memory storage** — HashMap-backed key-value store
- **Commands** — `PING` → `PONG`, `ECHO <msg>` → echoes back

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
```

Or use `nc` / telnet and type raw RESP:

```
*1\r\n$4\r\nPING\r\n
```

## Project layout

```
src/
├── main.rs          # Server entrypoint, connection handling
├── resp.rs          # RESP protocol parsing
├── resp_results.rs  # RESP error types
├── storage.rs       # Command execution (PING, ECHO, etc.)
├── storage_result.rs
└── server.rs
```

## Dependencies

- **tokio** — async runtime (TCP, spawn)

## License

MIT
