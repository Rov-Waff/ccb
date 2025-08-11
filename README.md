# 🎨 CCB Logger

[![Rust](https://img.shields.io/badge/rust-stable-brightgreen.svg)](https://www.rust-lang.org/)
[![Latest version](https://img.shields.io/crates/v/ccb.svg)](https://crates.io/crates/ccb)
[![Documentation](https://docs.rs/ccb/badge.svg)](https://docs.rs/ccb)
![License](https://img.shields.io/crates/l/ccb.svg)

> 🚀 A beautiful, terminal-focused structured logger for Rust

CCB brings elegance and visual appeal to the Rust ecosystem. It is designed for command-line interface (CLI) applications that want to achieve beautiful, readable, and structured log output.

## ✨ Features

- 🎯 **Semantic Log Levels**: Trace, Debug, Info, Warn, Error with four-character alignment
- 🌈 **Automatic Colors**: Beautiful colored output with smart terminal detection  
- ⏰ **Precise Timestamps**: High-precision timestamps in `2006-01-02 03:04:05.789` format
- 🔗 **Chainable Context**: Add structured key-value pairs with `with(key, value)`
- 🛠️ **Simple Macros**: Easy-to-use macros with variadic arguments support
- 🎛️ **Global Logger**: Set and use a global logger instance across your application
- 📱 **Terminal Friendly**: No icons, maximum compatibility across terminals
- ⚡ **Zero Config**: Works beautifully out of the box with sensible defaults

## 🚀 Quick Start

Add CCB Logger to your `Cargo.toml`:

```toml
[dependencies]
ccb = "0.1.0"
```

### Basic Usage

```rust
use ccb::{info, warn, error, debug, trace};

fn main() {
    // Simple logging
    info!("Application started");
    warn!("This is a warning");
    error!("Something went wrong");
    
    // With structured fields
    info!("User login", "user_id", "12345", "ip", "192.168.1.100");
    error!("Database error", "table", "users", "error", "connection timeout");
}
```

### Custom Logger Configuration

```rust
use ccb::{Logger, Level, set_global_logger};

fn main() {
    // Create a custom logger
    let logger = Logger::new()
        .with_level(Level::Debug)
        .with_colors(true)
        .with_timestamp(true)
        .with("service", "my-app")
        .with("version", "1.0.0");
    
    // Set as global logger
    set_global_logger(logger);
    
    // Now all macro calls will use the configured logger
    debug!("Debug message with context");
    info!("Request processed", "method", "GET", "path", "/api/users");
}
```

### Advanced Usage

```rust
use ccb::{Logger, Level, Config};

fn main() {
    // Custom configuration
    let config = Config {
        level: Level::Trace,
        use_colors: false,  // Disable colors for CI/CD
        show_timestamp: true,
    };
    
    let logger = Logger::with_config(config)
        .with("component", "auth")
        .with("environment", "production");
    
    // Direct logger usage
    logger.trace("Entering function", &[("fn", "authenticate")]);
    logger.info("Authentication successful", &[("user", "alice")]);
    logger.error("Rate limit exceeded", &[("ip", "192.168.1.1"), ("attempts", "10")]);
}
```

## 📊 Output Examples

```
2024-01-15 14:30:25.1234 INFO Application started
2024-01-15 14:30:25.1235 WARN Configuration file not found path=config.toml
2024-01-15 14:30:25.1236 INFO User login user_id=12345 ip=192.168.1.100
2024-01-15 14:30:25.1237 ERRO Database connection failed error=timeout retry_count=3
2024-01-15 14:30:25.1238 DEBG Cache hit key=user:12345 ttl=300
```

## 🎯 Log Levels

CCB supports five log levels with four-character alignment:

| Level | Code | Color  | Description |
|-------|------|--------|-------------|
| Trace | `TRCE` | Cyan   | 🔍 Detailed tracing information |
| Debug | `DEBG` | Blue   | 🐛 Debug information for developers |  
| Info  | `INFO` | Green  | ℹ️ General information messages |
| Warn  | `WARN` | Yellow | ⚠️ Warning messages |
| Error | ERRO | Red    | ❌ Error conditions |

## 🔧 Configuration Options

### Logger Methods

- `with_level(level)` - Set minimum log level
- `with_colors(bool)` - Enable/disable colored output  
- `with_timestamp(bool)` - Show/hide timestamps
- `with(key, value)` - Add context key-value pair

### Environment Detection

CCB automatically detects if output is going to a terminal and enables colors accordingly. You can override this behavior:

```rust
let logger = Logger::new().with_colors(false); // Force disable colors
```

## 🧪 Testing

Run the test suite:

```bash
cargo test
```

Run tests with output:

```bash
cargo test -- --nocapture
```

## 📚 Examples

Check out the `examples/` directory for more usage patterns:

```bash
cargo run --example basic_usage
```

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

1. 🍴 Fork the repository
2. 🌟 Create your feature branch (`git checkout -b feature/amazing-feature`)
3. ✅ Commit your changes (`git commit -m 'Add some amazing feature'`)
4. 📤 Push to the branch (`git push origin feature/amazing-feature`)
5. 🔄 Open a Pull Request

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- Inspired by [charmbracelet/log](https://github.com/charmbracelet/log) ❤️
- CCB has **no real meaning**, the name was given by a friend
- Built with ❤️ for the Rust community
- Thanks to all contributors! 🎉

## 🔗 Related Projects

- [charmbracelet/log](https://github.com/charmbracelet/log) - The original Go implementation
- [env_logger](https://crates.io/crates/env_logger) - Simple logger controlled via environment
- [tracing](https://crates.io/crates/tracing) - Application-level tracing framework
