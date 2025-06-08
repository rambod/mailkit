use mailkit::EmailSender;
use std::env;

#[test]
fn new_valid_email() {
    unsafe { env::set_var("MAILKIT_TEMPLATE_DIR", "tests/templates"); }
    let sender = EmailSender::new(
        "user@example.com",
        "smtp.example.com",
        "password",
        25,
        1,
        true,
    );
    assert!(sender.is_ok());
}

#[test]
fn new_invalid_email() {
    unsafe { env::set_var("MAILKIT_TEMPLATE_DIR", "tests/templates"); }
    let sender = EmailSender::new(
        "invalid",
        "smtp.example.com",
        "password",
        25,
        1,
        true,
    );
    assert!(sender.is_err());
}

#[test]
fn from_env_missing() {
    unsafe {
        env::remove_var("EMAIL");
        env::remove_var("SMTP_SERVER");
        env::remove_var("EMAIL_PASSWORD");
        env::remove_var("SMTP_PORT");
    }
    let res = EmailSender::from_env();
    assert!(res.is_err());
}
