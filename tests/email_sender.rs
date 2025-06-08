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

/// This test will actually send a real email using Gmail SMTP.
/// You must fill in your real credentials and recipient!
#[ignore]
#[test]
fn send_real_gmail() {
    // Fill these in before running!
    let your_gmail = "YOUR_GMAIL@gmail.com";
    let app_password = "YOUR_GMAIL_APP_PASSWORD"; // App Password, not your Gmail password!
    let dest_email = "DESTINATION_EMAIL@gmail.com"; // Where you want to send

    unsafe { env::set_var("MAILKIT_TEMPLATE_DIR", "tests/templates"); }

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

    // No CC, no BCC, no attachments, TLS flag set to true, plain text
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
