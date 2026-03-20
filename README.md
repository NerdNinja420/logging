# Logging - Colored Terminal Logging

![Screenshot](./assets/logging.png)

## Usage

```rust
use logging::{info, warn, err, rcmd};

fn main() {
    info("Application started");
    warn("This is a warning");
    err("Something went wrong");
    rcmd("Running command");
}
```

## Log Levels

| Function | Color | Purpose |
|----------|-------|---------|
| `info`   | Blue  | General information |
| `warn`   | Yellow | Warnings |
| `err`    | Red   | Errors |
| `rcmd`   | Green | Command output |

## Requirements

- Rust 1.75 or later

## Installation

### As a Dependency

```toml
[dependencies]
logging = { git = "git@github.com:NerdNinja420/logging.git", branch = "main" }
```

## License

This project is licensed under the MIT License.
