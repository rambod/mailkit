# MailKit Rust Library

MailKit is a modern Rust library for sending emails (sync and async), supporting HTML, attachments, templates (Tera), CC/BCC, and more.  
Built with [lettre](https://lettre.rs), Tera, and modern Rust practices.

Author: Rambod  
Website: [rambod.net](https://rambod.net)

---

## Features

- Simple sync and async email sending (SMTP)
- Async methods require a Tokio runtime
- HTML or plain text body support
- Attachments (any file type)
- Tera template support for dynamic email rendering
- Environment variable configuration for secrets
- CC/BCC
- Bulk sending
- Input email validation (built-in, no external crate)
- Minimal dependencies
- Fully customizable
- **No IMAP support (by design for now)**

---

## Dependencies

- lettre
- tera
- Built-in `SimpleLogger` with lightweight macros
- serde (for templates)

## Quick Start

### 1. Add to `Cargo.toml`:

```toml
lettre = { version = "0.11", features = ["smtp-transport", "tokio1"] }
tera = "1.17"
serde = { version = "1.0", features = ["derive"] }
```

### 2. Example Usage

```rust
use mailkit::{EmailSender, SimpleLogger};
use std::env;

env::set_var("EMAIL", "your@email.com");
env::set_var("EMAIL_PASSWORD", "yourpassword");
env::set_var("SMTP_SERVER", "smtp.yourprovider.com");
env::set_var("SMTP_PORT", "587");

SimpleLogger::init().unwrap();
let sender = EmailSender::from_env().unwrap();

sender.send(
    ["destination@email.com"],          // Recipients
    "Subject",
    "Hello world!",
    None,                              // CC
    None,                              // BCC
    None,                              // Attachments
    false,                             // Use TLS (true = TLS, false = STARTTLS)
    false                              // Is HTML
).unwrap();
```

### 3. HTML or Tera Template Example

```rust
use mailkit::json;
let tera_ctx = json!({
    "username": "Rambod",
    "link": "https://rambod.net"
});

sender.send_template(
    "user@client.com",
    "Welcome!",
    "welcome.html",    // Path inside ./templates/
    &tera_ctx,
    None,              // CC
    None,              // BCC
    None,              // Attachments
    false               // Use TLS (true = TLS, false = STARTTLS)
).unwrap();
```

### 4. Bulk Send Example

```rust
sender.send_bulk(
    vec!["a@email.com".into(), "b@email.com".into()],
    "Subject for all",
    "Content for all.",
    None,
    None,
    None,
    false,  // Use TLS (true = TLS, false = STARTTLS)
    false,
).unwrap();
```

### 5. Async Bulk Send Example

```rust
#[tokio::main]
async fn main() {
    sender.send_bulk_async(
        vec!["a@email.com".into(), "b@email.com".into()],
        "Subject for all",
        "Content for all.",
        None,
        None,
        None,
        false,
        false,
    ).await.unwrap();
}
```

Async functions like `send_async` and `send_bulk_async` require a Tokio runtime.

Each recipient receives its own email, and any addresses provided in `cc` or
`bcc` are included on every message.

---

## Environment Variables

- `EMAIL`            — Sender email address
- `EMAIL_PASSWORD`   — SMTP password
- `SMTP_SERVER`      — SMTP server host (e.g., smtp.gmail.com)
- `SMTP_PORT`        — SMTP port (e.g., 587)
- `MAILKIT_TEMPLATE_DIR` — (optional) path to template directory (default: ./templates)

---

## Attachments

- Provide a slice of file paths (as `&[String]`) to `attachments`.
- Attachments will be sent as MIME octet-stream.
- When using async sending, files are read with `tokio::fs`.

---

## Logging

- MailKit provides `info!`, `warn!`, and `error!` macros built into the crate.
  Call `mailkit::SimpleLogger::init()` to enable simple console logging.

---

## About

- Author: Rambod
- Website: [rambod.net](https://rambod.net)
- License: MIT

Pull requests and issues welcome.
