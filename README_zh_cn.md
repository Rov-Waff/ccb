# 👴🐘 CCB Logger

[![Rust](https://img.shields.io/badge/rust-stable-brightgreen.svg)](https://www.rust-lang.org/)
[![Latest version](https://img.shields.io/crates/v/ccb.svg)](https://crates.io/crates/ccb)
[![Documentation](https://docs.rs/ccb/badge.svg)](https://docs.rs/ccb)
![License](https://img.shields.io/crates/l/ccb.svg)

> 🚀 一个漂亮的、以终端为中心的Rust结构化记录器

Rust笑传之Console Catches Bugs。它专为希望实现美观、可读和结构化日志输出的命令行界面（CLI）应用程序而设计。

## ✨ 特点

- 🎯 **多种日志级别**：CCB具有跟踪、调试、信息、警告、错误五种日志级别，并做了字符对齐
- 🌈 **自动颜色**：具有智能终端检测功能的美丽彩色输出
- ⏰ **精确时间**：采用`YYYY-MM-DD hh:mm:ss.nnn`格式的高精度时间
- 🔗 **可链接上下文**：使用`with(key，value)添加结构化键值对`
- 🛠️ **简单的宏**：提供了易于使用的宏，支持可变参数
- 🎛️ **全局日志记录器**：您可以在应用程序中设置和使用全局记录器实例
- 📱 **终端友好**：无图标，最大限度地兼容终端
- ⚡ **零配置**：使用合理的默认值，开箱即用，效果很好

## 🚀 快速开始

快把CCB日志框架加入到你的 `Cargo.toml` 里吧！

```toml
[dependencies]
ccb = "0.1.0"
```

### 基本用法

```rust
use ccb::{info, warn, error, debug, trace};

fn main() {
    // 输出简单的日志
    info!("Application started");
    warn!("This is a warning");
    error!("Something went wrong");
    
    // 输出结构化的日志
    info!("User login", "user_id", "12345", "ip", "192.168.1.100");
    error!("Database error", "table", "users", "error", "connection timeout");
}
```

### 配置自定义的日志记录器

```rust
use ccb::{Logger, Level, set_global_logger};

fn main() {
    // 创建一个日志记录器
    let logger = Logger::new()
        .with_level(Level::Debug)
        .with_colors(true)
        .with_timestamp(true)
        .with("service", "my-app")
        .with("version", "1.0.0");
    
    // 设置为全局日志记录器
    set_global_logger(logger);
    
    // 现在所有的宏都会使用这个日志记录器输出日志
    debug!("Debug message with context");
    info!("Request processed", "method", "GET", "path", "/api/users");
}
```

### 高级用法

```rust
use ccb::{Logger, Level, Config};

fn main() {
    // 自定义配置
    let config = Config {
        level: Level::Trace,
        use_colors: false,  // 由于CI/CD禁用了彩色输出
        show_timestamp: true,
    };
    
    let logger = Logger::with_config(config)
        .with("component", "auth")
        .with("environment", "production");
    
    // 直接使用日志记录器
    logger.trace("Entering function", &[("fn", "authenticate")]);
    logger.info("Authentication successful", &[("user", "alice")]);
    logger.error("Rate limit exceeded", &[("ip", "192.168.1.1"), ("attempts", "10")]);
}
```

## 📊 输出示例

```
2024-01-15 14:30:25.1234 INFO Application started
2024-01-15 14:30:25.1235 WARN Configuration file not found path=config.toml
2024-01-15 14:30:25.1236 INFO User login user_id=12345 ip=192.168.1.100
2024-01-15 14:30:25.1237 ERRO Database connection failed error=timeout retry_count=3
2024-01-15 14:30:25.1238 DEBG Cache hit key=user:12345 ttl=300
```

## 🎯 日志级别

CCB 支持五个均含有字符对齐的日志级别：

| Level | Code | Color  | Description |
|-------|------|--------|-------------|
| Trace | `TRCE` | 青色  | 🔍  详细的追踪数据的信息 |
| Debug | `DEBG` | 蓝色   | 🐛 给开发者看的Debug信息 |  
| Info  | `INFO` | 绿色  | ℹ️ 通用信息 |
| Warn  | `WARN` | 黄色 | ⚠️ 警告信息 |
| Error | `ERRO` | 红色    | ❌ 错误信息 |

## 🔧 配置项

### `Logger` 的方法

- `with_level(level)` - 设置输出的最低日志等级
- `with_colors(bool)` - 启用/禁用多色彩输出  
- `with_timestamp(bool)` - 显示/隐藏时间
- `with(key, value)` - 增加上下文键值对

### 环境检测

CCB会自动检测输出是否要发送到终端，并相应地使用不同的颜色。您可以覆盖此行为：

```rust
let logger = Logger::new().with_colors(false); // 强制仅有颜色输出
```

## 🧪 测试

使用这条命令运行测试

```bash
cargo test
```

运行带有输出的测试:

```bash
cargo test -- --nocapture
```

## 📚 示例

查看 `examples/` 文件夹获取更多用法:

```bash
cargo run --example basic_usage
```

## 🤝 贡献

欢迎贡献！请随意打PR。对于重大更改，请先打开一个Issue来讨论您想要更改的内容。

1. 🍴 Fork 这个仓库
2. 🌟 创建你的feature分支 (`git checkout -b feature/amazing-feature`)
3. ✅ 提交你的更改 (`git commit -m 'Add some amazing feature'`)
4. 📤 推送你的分支 (`git push origin feature/amazing-feature`)
5. 🔄 开一个PR

## 📄 许可证

此项目根据MIT许可证进行许可-有关详细信息，请查看[License](License)文件。

## 🙏 致谢

- 灵感来源于 [charmbracelet/log](https://github.com/charmbracelet/log) ❤️
- CCB 没有意义！它的名字是我的一个朋友起的
- 为Rust社区而构建
- 感谢所有的贡献者 🎉

## 🔗 相关项目

- [charmbracelet/log](https://github.com/charmbracelet/log) - 最初的Go实现
- [env_logger](https://crates.io/crates/env_logger) - 通过环境变量控制的Logger
- [tracing](https://crates.io/crates/tracing) - 应用级追踪框架
