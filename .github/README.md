# 🏰 Taran

![CI](https://github.com/Shaqal7/Taran/workflows/CI/badge.svg)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE-MIT)
[![Rust](https://img.shields.io/badge/rust-1.75%2B-orange.svg)](https://www.rust-lang.org)

> **Taran** (тара́н) — a high-performance load testing tool written in Rust.
> Modern alternative to JMeter, Gatling, and K6.

---

## Why Taran?

Existing load testing tools come with trade-offs: JMeter is GUI-heavy and resource-hungry, K6 uses Go's garbage collector which skews latency measurements, and Goose requires Rust compilation knowledge. **Taran** fills this gap:

- 🚀 **Zero GC, Zero Cost Abstractions** — Rust's ownership model delivers deterministic latency measurements with no garbage collection pauses
- 📊 **Accurate Metrics** — HDR Histogram-based percentiles with proper Coordinated Omission handling
- ⚡ **Async-First** — Powered by Tokio, supports tens of thousands of concurrent virtual users with minimal memory
- 📝 **Simple Configuration** — Declarative TOML scenarios, no code required for common cases
- 🔧 **Scriptable** — Rhai scripting engine for complex scenarios (coming soon)
- 📦 **Single Binary** — Statically linked, no runtime dependencies, easy CI/CD integration
- 🖥️ **Cross-Platform** — Linux, macOS, and Windows

## Architecture

Taran follows **Clean Architecture** principles with a modular Cargo workspace:

```
┌─────────────────────────────────────────────────────────┐
│                      CLI (clap)                         │
│            Argument parsing & composition root          │
└──────────────────────┬──────────────────────────────────┘
                       │
┌──────────────────────▼──────────────────────────────────┐
│                Config Loader (TOML)                      │
│          Scenario parsing & validation                   │
└──────────────────────┬──────────────────────────────────┘
                       │
┌──────────────────────▼──────────────────────────────────┐
│              Execution Engine (Tokio)                     │
│  ┌─────────┐ ┌─────────┐ ┌─────────┐ ┌─────────┐      │
│  │  VU #1  │ │  VU #2  │ │  VU #3  │ │  VU #N  │      │
│  │ (task)  │ │ (task)  │ │ (task)  │ │ (task)  │      │
│  └────┬────┘ └────┬────┘ └────┬────┘ └────┬────┘      │
│       └────────────┴──────────┴────────────┘            │
│                       │                                  │
│  ┌────────────────────▼────────────────────────────┐    │
│  │         Protocol Clients (reqwest/hyper)          │    │
│  │    HTTP/1.1 · HTTP/2 · gRPC · WebSocket · TCP    │    │
│  └────────────────────┬────────────────────────────┘    │
└───────────────────────┼─────────────────────────────────┘
                        │
┌───────────────────────▼─────────────────────────────────┐
│             Metrics Collector (lock-free)                 │
│  HDR Histogram · Throughput · Error rates · Percentiles  │
└──────────┬──────────────────────────────┬───────────────┘
           │                              │
┌──────────▼──────────┐    ┌──────────────▼──────────────┐
│  Real-time Reporter │    │     Export / Sink            │
│  Console · TUI      │    │  JSON · CSV · HTML          │
└─────────────────────┘    └─────────────────────────────┘
```

### Workspace Structure

| Crate | Purpose |
|---|---|
| [`taran-cli`](taran-cli/) | Binary entry point — CLI parsing (clap), composition root |
| [`taran-core`](taran-core/) | Execution engine — test runner, VU lifecycle, trait definitions |
| [`taran-config`](taran-config/) | TOML scenario parsing and validation (serde) |
| [`taran-metrics`](taran-metrics/) | Metrics collection — percentiles, latency tracking |
| [`taran-protocols`](taran-protocols/) | Protocol clients — HTTP/1.1, HTTP/2 |
| [`taran-report`](taran-report/) | Report generation — console output |
| [`taran-script`](taran-script/) | Scripting engine — Rhai integration (planned) |

Dependencies flow top-down: `cli → core → {config, metrics, protocols, script}`, `report → metrics`. Lower-level crates never depend on higher-level ones.

## Quick Start

### Installation

Build from source (requires [Rust 1.75+](https://rustup.rs)):

```bash
git clone https://github.com/Shaqal7/Taran.git
cd Taran
cargo install --path taran-cli
```

### Create a Scenario

Create a file `test.toml`:

```toml
[scenario]
name = "Basic HTTP Test"
description = "Simple HTTP GET and POST scenario"

[load_profile]
type = "constant"
users = 1
duration = "10s"

[[steps]]
name = "GET Homepage"
protocol = "http"
method = "GET"
url = "https://httpbin.org/get"

[steps.assertions]
status = 200
max_response_time = "2000ms"

[[steps]]
name = "POST Data"
protocol = "http"
method = "POST"
url = "https://httpbin.org/post"
body = '{"test": "data"}'

[steps.headers]
"Content-Type" = "application/json"

[steps.assertions]
status = 200
```

### Run the Test

```bash
taran run test.toml
```

### Validate a Scenario (without running)

```bash
taran validate test.toml
```

### CLI Options

```bash
taran run test.toml              # Run a load test
taran run test.toml -u 50        # Override virtual user count
taran run test.toml -d 30s       # Override duration
taran run test.toml --verbose    # Enable debug logging
taran validate test.toml         # Validate scenario file
taran --help                     # Show help
taran --version                  # Show version
```

## Scenario Configuration

Scenarios are defined in TOML with the following structure:

### Load Profiles

```toml
# Constant load — fixed number of users
[load_profile]
type = "constant"
users = 100
duration = "60s"
ramp_up = "10s"          # optional ramp-up period

# Ramp — linear increase from→to
[load_profile]
type = "ramp"
from = 10
to = 200
duration = "120s"

# Stepped — incremental stages
[load_profile]
type = "stepped"
[[load_profile.steps]]
users = 50
duration = "30s"
[[load_profile.steps]]
users = 100
duration = "60s"

# Spike — sudden burst
[load_profile]
type = "spike"
baseline = 10
peak = 500
spike_duration = "10s"
total_duration = "120s"
```

### Steps & Assertions

```toml
[[steps]]
name = "API Request"
protocol = "http"
method = "POST"
url = "https://api.example.com/data"
body = '{"key": "value"}'

[steps.headers]
"Content-Type" = "application/json"
"Authorization" = "Bearer token123"

[steps.assertions]
status = 200
max_response_time = "500ms"
body_contains = "success"

[steps.extract]
token = { from = "body", type = "jsonpath", expr = "$.token" }
```

## Current Status

Taran is in **Phase 0 (Foundation)** — the core skeleton is functional with an end-to-end flow:

### ✅ Implemented

- TOML-based scenario configuration with validation
- HTTP/1.1 and HTTP/2 protocol support (via reqwest + rustls)
- Load profile definitions (constant, ramp, stepped, spike)
- Request assertions (status code, response time, body contains)
- Metrics collection with percentile calculation (p50, p95, p99)
- Console report output with summary statistics
- CLI with `run` and `validate` commands
- Cross-platform CI (Linux, macOS, Windows)
- Variable extraction definitions (JSONPath, regex)

### 🚧 Planned

| Phase | Features |
|---|---|
| **Phase 1** | Multi-VU execution, scheduler, open/closed loop models, data correlation |
| **Phase 2** | gRPC (tonic), WebSocket (tokio-tungstenite), raw TCP/UDP |
| **Phase 3** | HDR Histogram, lock-free metrics, real-time TUI dashboard (ratatui), HTML/JSON/CSV export |
| **Phase 4** | Rhai scripting engine for complex scenarios without recompilation |
| **Phase 5** | Distributed mode — controller/worker architecture |
| **Phase 6** | Plugin system (dynamic libraries / WASM), JMeter/K6 converters |

See [PLAN.md](docs/PLAN.md) for the full roadmap with detailed descriptions and milestones.

## Technology Stack

| Component | Technology | Purpose |
|---|---|---|
| Async runtime | [Tokio](https://tokio.rs) | Asynchronous execution engine |
| HTTP client | [reqwest](https://docs.rs/reqwest) + [hyper](https://hyper.rs) | HTTP/1.1 and HTTP/2 support |
| TLS | [rustls](https://docs.rs/rustls) | Pure-Rust TLS (no OpenSSL dependency) |
| CLI | [clap](https://docs.rs/clap) (derive) | Command-line argument parsing |
| Config | [serde](https://serde.rs) + [toml](https://docs.rs/toml) | TOML scenario deserialization |
| Scripting | [Rhai](https://rhai.rs) | Embedded scripting engine |
| Metrics | [hdrhistogram](https://docs.rs/hdrhistogram) | Precise latency histograms |
| Logging | [tracing](https://docs.rs/tracing) | Structured, async-aware logging |
| Error handling | [thiserror](https://docs.rs/thiserror) + [anyhow](https://docs.rs/anyhow) | Typed errors (libs) / contextual errors (binary) |
| Testing | [wiremock](https://docs.rs/wiremock) | HTTP mock server for tests |

## Development

### Prerequisites

- [Rust 1.75+](https://rustup.rs) (stable toolchain)

### Build

```bash
cargo build                       # Debug build
cargo build --release             # Optimized release build
```

### Test

```bash
cargo test --workspace            # Run all tests
cargo test -p taran-config        # Test a specific crate
```

### Lint & Format

```bash
cargo fmt --all -- --check        # Check formatting
cargo clippy --all-targets --all-features -- -D warnings  # Lint
```

### Project Conventions

- **Zero `unwrap()`/`expect()` in production code** — enforced by Clippy (`deny`)
- **`thiserror`** for typed errors in library crates, **`anyhow`** only in the binary
- **Conventional Commits** for git messages (e.g., `feat(core): add ramp-up scheduler`)
- **Strict Clippy** — `all`, `pedantic`, and `nursery` lint groups enabled as warnings

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

Before submitting, make sure:

```bash
cargo fmt --all -- --check                                   # Formatting
cargo clippy --all-targets --all-features -- -D warnings     # Linting
cargo test --workspace                                       # Tests pass
```

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT license ([LICENSE-MIT](LICENSE-MIT))

at your option.