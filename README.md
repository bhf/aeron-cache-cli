# Aeron Cache CLI
![img.png](https://raw.githubusercontent.com/bhf/aeron-cache/refs/heads/main/docs/images/header.png)


[![CI](https://github.com/bhf/aeron-cache-cli/actions/workflows/test.yml/badge.svg)](https://github.com/bhf/aeron-cache-cli/actions/workflows/test.yml)
[![Near Cache CI](https://github.com/bhf/aeron-cache-cli/actions/workflows/near-test.yml/badge.svg)](https://github.com/bhf/aeron-cache-cli/actions/workflows/near-test.yml)

A CLI on top of the [Aeron Cache](https://github.com/bhf/aeron-cache) REST API.

```bash
brew tap bhf/aeron-cache-cli
brew install aeron-cache-cli
```

Built in Rust 🦀 using:

* [Clap](https://github.com/clap-rs/clap)
* [Dialoguer](https://github.com/console-rs/dialoguer)
* [Reqwest](https://github.com/seanmonstar/reqwest)
* [Serde](https://github.com/serde-rs/serde)


[See it in action](https://aeron-cache.pages.dev/see-it-in-action/)


By default, the CLI assumes that your backend cache service is reachable at `http://localhost:7070/api/v1`. If you need to override the API base URL, use the `--api-url` flag or the `AERON_CACHE_API_URL` environment variable.

## Available commands

It exposes the following commands:

### Cache commands
- `create <name>`: Create a new cache
- `delete <name>`: Delete a cache
- `get-cache <name>`: Get all items from a cache
- `clear-cache <name>`: Clear all items from a cache
- `insert <name> <key> <value>`: Insert an item into a cache
- `insert-timed <name> <key> <value> <ttl>`: Insert an item with a time-to-live (milliseconds) into a cache
- `get <name> <key>`: Get an item from a cache
- `remove <name> <key>`: Remove an item from a cache
- `cancel-removal <name> <key>`: Cancel a scheduled removal of an item
- `list-caches`: List all available caches and their item count
- `stats`: Get global cache statistics

### Counter commands
- `create-counter-cache <name>`: Create a new counter cache
- `delete-counter-cache <name>`: Delete a counter cache
- `get-counter-cache <name>`: Get all counters from a counter cache
- `clear-counter-cache <name>`: Clear all counters from a counter cache
- `put-counter <name> <key> <value>`: Put a counter into a counter cache
- `put-timed-counter <name> <key> <value> <ttl>`: Put a counter with a time-to-live (milliseconds) into a counter cache
- `get-counter <name> <key>`: Get a single counter from a counter cache
- `delete-counter <name> <key>`: Delete a single counter from a counter cache
- `increment-counter <name> <key> [amount]`: Increment a counter (defaults to 1)
- `decrement-counter <name> <key> [amount]`: Decrement a counter (defaults to 1)
- `set-counter <name> <key> <value>`: Set a counter to a given value
- `cancel-counter-removal <name> <key>`: Cancel a scheduled removal of a counter
- `list-counter-caches`: List all available counter caches and their item count
- `counter-stats`: Get global counter cache statistics

### Near cache commands

These manage the server's local near cache and require the backend to be started with near caching enabled (`NEAR_CACHE_ENABLED=true`). When enabled, reads are served transparently from the near cache on the usual `get`/`get-cache`/`get-counter`/`get-counter-cache` commands; the commands below manage which caches are kept warm.

- `prime-near-cache <name>`: Prime a near cache so its local copy is warmed up front
- `stop-near-cache <name>`: Stop near caching a cache and drop its local copy
- `list-near-caches`: List all caches currently held warm in the near cache
- `prime-near-counter-cache <name>`: Prime a near counter cache so its local copy is warmed up front
- `stop-near-counter-cache <name>`: Stop near caching a counter cache and drop its local copy
- `list-near-counter-caches`: List all counter caches currently held warm in the near cache

Use the `--help` flag anytime for more information.

Example: `cargo run -- create mycache`
Example: `cargo run -- --api-url http://otherhost:7070/api/v1 stats`

### JSON output

Pass the global `--json` flag to print the server's raw JSON response instead of human-readable text, e.g. `ac-cli --json get-cache mycache`. Server-side errors are printed as JSON too.
