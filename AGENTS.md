# Krystal API Client - Agent Guidelines

## Build Commands
- Build: `cargo build`
- Test: `cargo test` (single test: `cargo test test_name`)
- Check: `cargo check`
- Lint: `cargo clippy -- -D warnings`
- Format: `cargo fmt`

## Code Style
- **Imports**: Group as `std`, external crates, then local (`crate::`) modules
- **Error Handling**: Use custom `Result<T>` type alias from `error.rs`, implement `thiserror::Error`
- **Async**: All API methods are async, use `tokio` runtime with `#[tokio::main]`
- **Naming**: snake_case for functions/variables, PascalCase for types, SCREAMING_SNAKE_CASE for constants
- **File Headers**: Include `// file:`, `// description:`, and `// docs_reference:` comments
- **Documentation**: Use `///` for public items with examples when appropriate
- **Builder Pattern**: Use for complex queries (see `query.rs`)
- **Type Safety**: Prefer enums over strings for API parameters
- **Dependencies**: Use clap (derive), reqwest (json+rustls), serde (derive), dotenvy, env_logger, thiserror
- **Tests**: Place unit tests in same file with `#[cfg(test)]` module
- **Logging**: Use `env_logger` and `log` crate for structured logging
- **NO COMMENTS**: Do not add implementation comments unless explicitly requested