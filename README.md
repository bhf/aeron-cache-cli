# Aeron Cache CLI
![img.png](https://raw.githubusercontent.com/bhf/aeron-cache/refs/heads/main/docs/images/header.png)


[![CI](https://github.com/bhf/aeron-cache-cli/actions/workflows/test.yml/badge.svg)](https://github.com/bhf/aeron-cache-cli/actions/workflows/test.yml)

A CLI on top of the [Aeron Cache](https://github.com/bhf/aeron-cache) REST API.

Built in Rust 🦀 using:

* [Clap](https://github.com/clap-rs/clap)
* [Dialoguer](https://github.com/console-rs/dialoguer)
* [Reqwest](https://github.com/seanmonstar/reqwest)
* [Serde](https://github.com/serde-rs/serde)


[![asciinema demo](https://asciinema.org/a/JPbih6gJuwFHZxhj9Op64UKgN.svg)](https://asciinema.org/a/JPbih6gJuwFHZxhj9Op64UKgN)


By default, the CLI assumes that your backend cache service is reachable at `http://localhost:7070/api/v1`. If you need to override the API base URL, use the `--api-url` flag or the `AERON_CACHE_API_URL` environment variable.

## Available commands

It exposes the following commands:
- `create <name>`: Create a new cache
- `delete <name>`: Delete a cache
- `get-cache <name>`: Get all items from a cache
- `clear-cache <name>`: Clear all items from a cache
- `insert <name> <key> <value>`: Insert an item into a cache
- `get <name> <key>`: Get an item from a cache
- `remove <name> <key>`: Remove an item from a cache
- `list-caches`: List all available caches and their item count
- `stats`: Get global cache statistics

Use the `--help` flag anytime for more information.

Example: `cargo run -- create mycache`
Example: `cargo run -- --api-url http://otherhost:7070/api/v1 stats`
