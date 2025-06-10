use mailkit::EmailSender;
use serial_test::serial;
use std::env;
use std::ffi::OsStr;

fn set_var<K: AsRef<OsStr>, V: AsRef<OsStr>>(key: K, val: V) {
    unsafe { env::set_var(key, val) }
}

fn remove_var<K: AsRef<OsStr>>(key: K) {
    unsafe { env::remove_var(key) }
}

#[test]
#[serial]
fn new_valid_email() {
    set_var("MAILKIT_TEMPLATE_DIR", "tests/templates");
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
#[serial]
fn new_invalid_email() {
    set_var("MAILKIT_TEMPLATE_DIR", "tests/templates");
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
#[serial]
fn from_env_missing() {
    remove_var("EMAIL");
    remove_var("SMTP_SERVER");
    remove_var("EMAIL_PASSWORD");
    remove_var("SMTP_PORT");
    let res = EmailSender::from_env();
    assert!(res.is_err());
}

#[test]
#[serial]
fn from_env_valid() {
    set_var("MAILKIT_TEMPLATE_DIR", "tests/templates");
    set_var("EMAIL", "user@example.com");
    set_var("SMTP_SERVER", "smtp.example.com");
    set_var("EMAIL_PASSWORD", "password");
    set_var("SMTP_PORT", "25");

    let res = EmailSender::from_env();
    assert!(res.is_ok());
}

/// This test will actually send a real email using Gmail SMTP.
/// You must fill in your real credentials and recipient!
#[ignore]
#[test]
#[serial]
fn send_real_gmail() {
    // Fill these in before running!
    let your_gmail = "YOUR_GMAIL@gmail.com";
    let app_password = "YOUR_GMAIL_APP_PASSWORD"; // App Password, not your Gmail password!
    let dest_email = "DESTINATION_EMAIL@gmail.com"; // Where you want to send

    set_var("MAILKIT_TEMPLATE_DIR", "tests/templates");

    let sender = EmailSender::new(
        your_gmail,
        "smtp.gmail.com",
        app_password,
        587,
        10,    // timeout seconds
        true,  // validate_emails
    ).expect("Failed to create EmailSender");

    let recipients = vec![dest_email.to_string()];
    let subject = "MailKit Rust Test";
    let body = "This is a test email sent by MailKit integration test.";

    // No CC, no BCC, no attachments, TLS flag set to true (direct TLS), plain text
    let result = sender.send(
        recipients,
        subject,
        body,
        None,     // CC
        None,     // BCC
        None,     // Attachments
        true,     // Use TLS
        false,    // HTML
    );

    assert!(result.is_ok(), "Failed to send email: {:?}", result.err());
}
