# Taran

![CI](https://github.com/Shaqal7/Taran/workflows/CI/badge.svg)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE-MIT)

> High-performance load testing tool written in Rust

Taran is a modern load testing tool designed to replace JMeter, Gatling, and K6. Built with Rust, it offers:

- 🚀 **High Performance**: Rust's zero-cost abstractions and async runtime
- 📊 **Accurate Metrics**: Proper handling of Coordinated Omission
- 🔧 **Easy to Use**: Simple TOML configuration files
- 📦 **Single Binary**: No dependencies, easy deployment
- 🌐 **Multiple Protocols**: HTTP/1.1, HTTP/2 (gRPC, WebSocket coming soon)

## Quick Start

### Installation

Download the latest release from the [releases page](https://github.com/Shaqal7/Taran/releases).

Or build from source:

```bash
cargo install --path taran-cli
```

### Usage

Create a scenario file `test.toml`:

```toml
[scenario]
name = "Basic HTTP Test"

[load_profile]
type = "constant"
users = 100
duration = "60s"

[[steps]]
name = "GET Homepage"
protocol = "http"
method = "GET"
url = "https://example.com/"

[steps.assertions]
status = 200
max_response_time = "500ms"
```

Run the test:

```bash
taran run test.toml
```

## Features (Phase 0 - MVP)

- ✅ TOML-based scenario configuration
- ✅ HTTP/1.1 and HTTP/2 support
- ✅ Basic load profiles (constant load)
- ✅ Assertions (status code, response time)
- ✅ Metrics collection with percentiles
- ✅ Console reporting
- ✅ Cross-platform binaries (Linux, macOS, Windows)

## Roadmap

See [PLAN.md](docs/PLAN.md) for the full development roadmap.

- **Phase 0**: Foundation ✅ (You are here)
- **Phase 1**: Load profiles and VU management
- **Phase 2**: Additional protocols (gRPC, WebSocket)
- **Phase 3**: Advanced metrics and TUI dashboard
- **Phase 4**: Scripting with Rhai
- **Phase 5**: Distributed mode
- **Phase 6**: Plugin system and ecosystem

## Development

Build the project:

```bash
cargo build
```

Run tests:

```bash
cargo test
```

Format and lint:

```bash
cargo fmt
cargo clippy
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT license ([LICENSE-MIT](LICENSE-MIT))

at your option.