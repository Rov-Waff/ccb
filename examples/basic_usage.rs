//! # CCB Logger - Basic Usage Example
//!
//! This example demonstrates how to use the CCB logging macros in a real application.
//! It shows:
//! - Using `trace!`, `debug!`, `info!`, `warn!`, `error!` macros
//! - Adding structured key-value context
//! - Setting a custom global logger
//! - Log level filtering

use ccb::{debug, error, info, set_global_logger, trace, warn, Level, Logger};

fn main() {
    println!("🚀 Starting CCB Logger Basic Usage Example\n");

    // ————————————————————————————————————————
    // Example 1: Default Global Logger (Info level and above)
    // ————————————————————————————————————————
    println!("📝 Example 1: Using default logger via macros");
    info!("Application started successfully");
    warn!("Configuration file not found, using defaults", "file", "config.toml");
    error!("Failed to connect to database", "host", "localhost", "port", "5432");
    // trace! and debug! won't show (default level is Info)
    trace!("This trace message will NOT appear");
    debug!("This debug message will NOT appear");

    println!();

    // ————————————————————————————————————————
    // Example 2: Custom Global Logger (enable Trace level)
    // ————————————————————————————————————————
    println!("🔧 Example 2: Setting a custom global logger");
    let custom_logger = Logger::new()
        .with_level(Level::Trace) // Show all levels
        .with_colors(true)
        .with_timestamp(true)
        .with("service", "api-server")
        .with("version", "1.0.0");

    set_global_logger(custom_logger);

    trace!("Handling incoming request", "method", "GET", "path", "/health");
    debug!("Query parameters parsed", "user_id", "123");
    info!("User authenticated", "user", "alice");
    warn!("Rate limit approaching", "limit", "90", "max", "100");
    error!("Failed to write to disk", "path", "/var/log/app.log", "err", "PermissionDenied");

    println!();

    // ————————————————————————————————————————
    // Example 3: Structured Logging with Key-Value Pairs
    // ————————————————————————————————————————
    println!("📦 Example 3: Rich structured logging");
    info!(
        "Payment processed successfully",
        "user_id", "usr-789",
        "amount", "99.99",
        "currency", "USD",
        "txn_id", "txn-12345"
    );

    error!(
        "User login failed",
        "username", "bob",
        "ip", "192.168.1.200",
        "attempt", "3",
        "reason", "invalid_otp"
    );

    println!();

    // ————————————————————————————————————————
    // Example 4: Log Level Filtering in Action
    // ————————————————————————————————————————
    println!("🎚️  Example 4: Filtering logs by level");
    let filtered_logger = Logger::new().with_level(Level::Warn);
    set_global_logger(filtered_logger);

    info!("This info message will be SILENTLY filtered out");
    debug!("This debug message is below threshold — skipped");
    warn!("⚠️  Warning: Disk space low", "used_percent", "92%");
    error!("🔥 Critical: Backup job failed", "job_id", "backup-001");

    println!("\n✅ Example completed. Check your terminal output for colorful, structured logs!");
}