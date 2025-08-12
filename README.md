# Throttles Service

[![Crates.io](https://img.shields.io/crates/v/throttle-ro)](https://crates.io/crates/throttle-ro)
[![Documentation](https://docs.rs/throttle-ro/badge.svg)](https://docs.rs/throttle-ro)
[![License](https://img.shields.io/crates/l/throttle-ro)](LICENSE-MIT)


A configurable rate limiting service for Rust applications, providing IP-based request throttling with cache-backed storage.

## Features

- 🚦 IP-based request throttling
- ⏱️ Configurable time windows
- 🔢 Multiple limit tiers
- 💾 Cache-backed storage (memory or persistent)
- ⚡ Optional async support

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
throttle-ro = "0.1"
```

## Usage

### Basic Example

```rust
use std::time::Duration;
use throttle_ro::ThrottlesService;
use cache_ro::Cache;

fn main() {
    let cache = Cache::new(Default::default()); // Configure properly in production
    let ip = "127.0.0.1".to_string();
    
    // Allow 5 requests per minute per IP
    let mut throttle = ThrottlesService::new(
        ip,
        5,
        Duration::from_secs(60),
        "api_"
    );

    if throttle.can_go(&cache) {
        throttle.hit(&cache);
        // Process request...
    } else {
        // Reject request
        println!("Rate limit exceeded!");
    }
}
```


## API Reference

Full documentation is available on [docs.rs](https://docs.rs/throttle-ro).


## Contributing

Contributions are welcome! Please open an issue or submit a PR for:
- New features
- Performance improvements
- Bug fixes


## License

Dual-licensed under [MIT](LICENSE-MIT) or [Apache 2.0](LICENSE-APACHE) at your option.