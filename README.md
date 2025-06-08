# MailKit Rust Library

MailKit is a modern Rust library for sending emails (sync and async), supporting HTML, attachments, templates (Tera), CC/BCC, and more.  
Built with [lettre](https://lettre.rs), Tera, and modern Rust practices.

Author: Rambod  
Website: [rambod.net](https://rambod.net)

---

## Features

- Simple sync and async email sending (SMTP)
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
- log, env_logger (optional)
- serde, serde_json (for templates)

## Quick Start

### 1. Add to `Cargo.toml`:

```toml
lettre = { version = "0.11", features = ["smtp-transport", "tokio1"] }
tera = "1.17"
log = "0.4"
env_logger = "0.9"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

### 2. Example Usage

```rust
use mailkit::EmailSender;
use std::env;

env::set_var("EMAIL", "your@email.com");
env::set_var("EMAIL_PASSWORD", "yourpassword");
env::set_var("SMTP_SERVER", "smtp.yourprovider.com");
env::set_var("SMTP_PORT", "587");

let sender = EmailSender::from_env().unwrap();

sender.send(
    ["destination@email.com"],          // Recipients
    "Subject",
    "Hello world!",
    None,                              // CC
    None,                              // BCC
    None,                              // Attachments
    false,                             // Use TLS (not used, kept for compat)
    false                              // Is HTML
).unwrap();
```

### 3. HTML or Tera Template Example

```rust
let tera_ctx = serde_json::json!({
    "username": "Rambod",
    "link": "https://rambod.net"
});

sender.send_template(
    "user@client.com",
    "Welcome!",
    "welcome.html",    // Path inside ./templates/
    &tera_ctx,
    None,              // CC
    None,              // Attachments
    false
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
    false,
    false,
).unwrap();
```

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

---

## Logging

- Uses `log` crate. Set up `env_logger` for debugging.

---

## About

- Author: Rambod
- Website: [rambod.net](https://rambod.net)
- License: MIT

Pull requests and issues welcome.
