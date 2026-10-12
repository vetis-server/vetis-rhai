# vetis-rhai

[![Crates.io downloads](https://img.shields.io/crates/d/vetis-rhai)](https://crates.io/crates/vetis-rhai) [![crates.io](https://img.shields.io/crates/v/vetis-rhai?style=flat-square)](https://crates.io/crates/vetis-rhai) [![Build Status](https://github.com/vetis-server/vetis-rhai/actions/workflows/rust.yml/badge.svg?event=push)](https://github.com/vetis-server/vetis-rhai/actions/workflows/rust.yml) ![Crates.io MSRV](https://img.shields.io/crates/msrv/vetis-rhai) [![Documentation](https://docs.rs/vetis-rhai/badge.svg)](https://docs.rs/vetis-rhai/latest/vetis-rhai) [![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/vetis-server/vetis-rhai/blob/main/LICENSE.md)  [![codecov](https://codecov.io/gh/vetis-server/vetis-rhai/graph/badge.svg?token=T0HSBAPVSI)](https://codecov.io/gh/vetis-server/vetis-rhai)

vetis-rhai added rhai scripting support to vetis server.

## Features

- Dynamic content generation with rhai scripting
- AST caching for fast script processing

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
vetis-rhai = "0.1.0"
```

### Runtime Features

Choose the appropriate runtime feature for your project:

- `runtime-tokio` (default): Use with Tokio runtime

## Configuration

### Using API

Configure static file serving using the builder pattern:

```rust,ignore
use vetis_rhai::RhaiPathConfig;

let config = RhaiPathConfig::builder()
    .uri("/script")
    .script_path("index.rhai")
    .build()?;
```

### vetis.yaml

```yaml
type: rhai_path
uri: /hello
script_path: index.rhai
```

### Configuration Options

#### rhai_path

- **script_path**: The path for rhai script file

## License

Licensed under either of

- Apache License, Version 2.0
  (LICENSE-APACHE or <https://www.apache.org/licenses/LICENSE-2.0>)
- MIT license
  (LICENSE-MIT or <https://opensource.org/licenses/MIT>)

at your option.

## Author

Rogerio Pereira Araujo <rogerio.araujo@gmail.com>
