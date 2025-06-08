use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::Duration;
use lettre::AsyncTransport;
use lettre::message::{header, Attachment, Mailbox, Message, MultiPart, SinglePart};
use lettre::transport::smtp::authentication::Credentials;
use lettre::{AsyncSmtpTransport, SmtpTransport, Tokio1Executor, Transport};
use log::{error, info, warn};
use tera::{Context, Tera};

use std::error::Error as StdError;
use std::fmt;
#[derive(Debug)]
pub enum MailkitError {
    Validation(String),
    Io(std::io::Error),
    Smtp(lettre::transport::smtp::Error),
    Tera(tera::Error),
    Build(lettre::error::Error),
    Address(lettre::address::AddressError),
    MissingEnvVar(&'static str),
}

impl fmt::Display for MailkitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MailkitError::Validation(msg) => write!(f, "Email validation failed: {}", msg),
            MailkitError::Io(err) => write!(f, "IO error: {}", err),
            MailkitError::Smtp(err) => write!(f, "SMTP error: {}", err),
            MailkitError::Tera(err) => write!(f, "Template error: {}", err),
            MailkitError::Build(err) => write!(f, "Build message error: {}", err),
            MailkitError::Address(err) => write!(f, "Address parse error: {}", err),
            MailkitError::MissingEnvVar(var) => write!(f, "Missing environment variable: {}", var),
        }
    }
}

impl StdError for MailkitError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            MailkitError::Io(err) => Some(err),
            MailkitError::Smtp(err) => Some(err),
            MailkitError::Tera(err) => Some(err),
            MailkitError::Build(err) => Some(err),
            MailkitError::Address(err) => Some(err),
            _ => None,
        }
    }
}

impl From<std::io::Error> for MailkitError {
    fn from(e: std::io::Error) -> Self {
        MailkitError::Io(e)
    }
}

impl From<lettre::transport::smtp::Error> for MailkitError {
    fn from(e: lettre::transport::smtp::Error) -> Self {
        MailkitError::Smtp(e)
    }
}

impl From<tera::Error> for MailkitError {
    fn from(e: tera::Error) -> Self {
        MailkitError::Tera(e)
    }
}

impl From<lettre::error::Error> for MailkitError {
    fn from(e: lettre::error::Error) -> Self {
        MailkitError::Build(e)
    }
}

impl From<lettre::address::AddressError> for MailkitError {
    fn from(e: lettre::address::AddressError) -> Self {
        MailkitError::Address(e)
    }
}

pub struct EmailSender {
    user_email: String,
    user_password: String,
    smtp_server: String,
    port: u16,
    timeout: Duration,
    validate_emails: bool,
    tera: Tera,
}

impl EmailSender {
    pub fn new<U: Into<String>>(
        user_email: U,
        smtp_server: U,
        user_password: U,
        port: u16,
        timeout_secs: u64,
        validate_emails: bool,
    ) -> Result<Self, MailkitError> {
        let email_str = user_email.into();
        let user_email = if validate_emails {
            Self::check_email(&email_str)?
        } else {
            email_str.clone()
        };

        let template_dir = PathBuf::from(
            env::var("MAILKIT_TEMPLATE_DIR").unwrap_or_else(|_| "templates".to_owned()),
        )
            .join("**/*");
        let tera = Tera::new(&template_dir.to_string_lossy())?;

        info!("EmailSender initialized for {}", user_email);

        Ok(Self {
            user_email,
            user_password: user_password.into(),
            smtp_server: smtp_server.into(),
            port,
            timeout: Duration::from_secs(timeout_secs),
            validate_emails,
            tera,
        })
    }

    pub fn from_env() -> Result<Self, MailkitError> {
        let user_email = env::var("EMAIL").map_err(|_| MailkitError::MissingEnvVar("EMAIL"))?;
        let server = env::var("SMTP_SERVER").map_err(|_| MailkitError::MissingEnvVar("SMTP_SERVER"))?;
        let password = env::var("EMAIL_PASSWORD").map_err(|_| MailkitError::MissingEnvVar("EMAIL_PASSWORD"))?;
        let port = env::var("SMTP_PORT")
            .unwrap_or_else(|_| "587".into())
            .parse()
            .unwrap_or(587);
        Self::new(user_email, server, password, port, 10, true)
    }

    fn check_email(addr: &str) -> Result<String, MailkitError> {
        fn is_valid_email(addr: &str) -> bool {
            let mut parts = addr.split('@');
            if let (Some(local), Some(domain), None) = (parts.next(), parts.next(), parts.next()) {
                !local.is_empty() && !domain.is_empty() && domain.contains('.')
            } else {
                false
            }
        }

        if is_valid_email(addr) {
            Ok(addr.to_lowercase())
        } else {
            error!("Invalid email address: {}", addr);
            Err(MailkitError::Validation(addr.into()))
        }
    }

    fn create_base_message<I, S>(
        &self,
        subject: &str,
        to: I,
        cc: Option<I>,
        bcc: Option<I>,
    ) -> Result<lettre::message::MessageBuilder, MailkitError>
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        let mut builder = Message::builder()
            .from(self.user_email.parse::<Mailbox>()?)
            .subject(subject);

        let to_addrs: Vec<Mailbox> = to
            .into_iter()
            .map(|rcpt| {
                let s = rcpt.into();
                if self.validate_emails {
                    Self::check_email(&s)
                } else {
                    Ok(s)
                }
            })
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .map(|s| s.parse())
            .collect::<Result<_, _>>()?;
        for m in &to_addrs {
            builder = builder.to(m.clone());
        }

        if let Some(cc_iter) = cc {
            let cc_addrs: Vec<Mailbox> = cc_iter
                .into_iter()
                .map(|c| {
                    let s = c.into();
                    if self.validate_emails {
                        Self::check_email(&s)
                    } else {
                        Ok(s)
                    }
                })
                .collect::<Result<Vec<_>, _>>()?
                .into_iter()
                .map(|s| s.parse())
                .collect::<Result<_, _>>()?;
            for m in &cc_addrs {
                builder = builder.cc(m.clone());
            }
        }

        if let Some(bcc_iter) = bcc {
            let bcc_addrs: Vec<Mailbox> = bcc_iter
                .into_iter()
                .map(|b| {
                    let s = b.into();
                    if self.validate_emails {
                        Self::check_email(&s)
                    } else {
                        Ok(s)
                    }
                })
                .collect::<Result<Vec<_>, _>>()?
                .into_iter()
                .map(|s| s.parse())
                .collect::<Result<_, _>>()?;
            for m in &bcc_addrs {
                builder = builder.bcc(m.clone());
            }
        }

        Ok(builder)
    }

    fn attach_files(
        &self,
        multipart: MultiPart,
        attachments: &[String],
    ) -> Result<MultiPart, MailkitError> {
        let mut mp = multipart;
        for path in attachments {
            let data = fs::read(path)?;
            let filename = Path::new(path)
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("attachment");
            let attachment = Attachment::new(filename.to_string())
                .body(data, lettre::message::header::ContentType::parse("application/octet-stream").unwrap());
            mp = mp.singlepart(attachment);
        }
        Ok(mp)
    }

    pub fn send<I, S>(
        &self,
        recipients: I,
        subject: &str,
        body: &str,
        cc: Option<I>,
        bcc: Option<I>,
        attachments: Option<&[String]>,
        _use_tls: bool,
        html: bool,
    ) -> Result<(), MailkitError>
    where
        I: IntoIterator<Item = S> + Clone,
        S: Into<String> + Clone,
    {
        let recipients_vec: Vec<String> = recipients.clone().into_iter().map(|x| x.into()).collect();
        info!("Sending email to: {}", recipients_vec.join(", "));

        let builder = self.create_base_message(subject, recipients.clone(), cc.clone(), bcc.clone())?;

        let content = if html {
            SinglePart::builder()
                .header(header::ContentType::parse("text/html; charset=utf-8").unwrap())
                .body(body.to_string())
        } else {
            SinglePart::plain(body.to_string())
        };

        let mut multipart = MultiPart::mixed().singlepart(content);

        if let Some(files) = attachments {
            multipart = self.attach_files(multipart, files)?;
        }

        let msg = builder.multipart(multipart)?;

        let creds = Credentials::new(self.user_email.clone(), self.user_password.clone());
        let mailer = SmtpTransport::relay(&self.smtp_server)?
            .credentials(creds)
            .port(self.port)
            .timeout(Some(self.timeout))
            .build();

        mailer.send(&msg)?;
        Ok(())
    }

    pub fn send_bulk(
        &self,
        recipients: Vec<String>,
        subject: &str,
        body: &str,
        cc: Option<Vec<String>>,
        bcc: Option<Vec<String>>,
        attachments: Option<&[String]>,
        use_tls: bool,
        html: bool,
    ) -> Result<(), MailkitError> {
        for rcpt in &recipients {
            info!("Bulk sending to {}", rcpt);

            self.send(
                vec![rcpt.clone()],
                subject,
                body,
                cc.clone(),
                bcc.clone(),
                attachments,
                use_tls,
                html,
            )?;
        }

        Ok(())
    }


    pub async fn send_async<I, S>(
        &self,
        recipients: I,
        subject: &str,
        body: &str,
        cc: Option<I>,
        bcc: Option<I>,
        attachments: Option<&[String]>,
        _use_tls: bool,
        html: bool,
    ) -> Result<(), MailkitError>
    where
        I: IntoIterator<Item = S> + Clone + Send + 'static,
        S: Into<String> + Clone + Send + 'static,
    {
        let recipients_vec: Vec<String> = recipients.clone().into_iter().map(|x| x.into()).collect();
        info!("Async sending to: {}", recipients_vec.join(", "));

        let builder = self.create_base_message(subject, recipients.clone(), cc.clone(), bcc.clone())?;

        let content = if html {
            SinglePart::builder()
                .header(header::ContentType::parse("text/html; charset=utf-8").unwrap())
                .body(body.to_string())
        } else {
            SinglePart::plain(body.to_string())
        };

        let mut multipart = MultiPart::mixed().singlepart(content);

        if let Some(files) = attachments {
            multipart = self.attach_files(multipart, files)?;
        }

        let msg = builder.multipart(multipart)?;

        let creds = Credentials::new(self.user_email.clone(), self.user_password.clone());
        let mailer: AsyncSmtpTransport<Tokio1Executor> =
            AsyncSmtpTransport::<Tokio1Executor>::relay(&self.smtp_server)?
                .credentials(creds)
                .port(self.port)
                .timeout(Some(self.timeout))
                .build();

        mailer.send(msg).await?;
        Ok(())
    }

    pub fn send_template<S: Into<String>>(
        &self,
        recipient: S,
        subject: &str,
        template_name: &str,
        context: &serde_json::Value,
        cc: Option<Vec<String>>,
        attachments: Option<&[String]>,
        use_tls: bool,
    ) -> Result<(), MailkitError> {
        let recipient_str = recipient.into();
        info!(
        "Sending templated email to {} using {}",
        recipient_str,
        template_name
    );
        let mut ctx = Context::new();
        if let Some(map) = context.as_object() {
            for (k, v) in map {
                ctx.insert(k, v);
            }
        }
        let body = self.tera.render(template_name, &ctx)?;

        // Only use first cc email, or None
        let cc_iter = cc.as_ref().and_then(|v| v.get(0).cloned().map(std::iter::once));
        let bcc_iter = None::<std::iter::Once<String>>;

        self.send(
            std::iter::once(recipient_str),
            subject,
            &body,
            cc_iter,
            bcc_iter,
            attachments,
            use_tls,
            true,
        )
    }

}

// Backward compatibility layer
pub struct SendAgent(pub EmailSender);

impl SendAgent {
    pub fn send_mail(
        &self,
        recipient_email: Vec<String>,
        subject: &str,
        message_body: &str,
        cc: Option<Vec<String>>,
        bcc: Option<Vec<String>>,
        attachments: Option<&[String]>,
        tls: bool,
    ) -> Result<(), MailkitError> {
        warn!("SendAgent is deprecated, use EmailSender instead");
        (self.0).send(
            recipient_email,
            subject,
            message_body,
            cc.clone(),
            bcc.clone(),
            attachments,
            tls,
            false,
        )
    }


    pub fn send_mail_with_template(
        &self,
        recipient_email: String,
        subject: &str,
        template_path: &str,
        template_vars: &serde_json::Value,
        cc: Option<Vec<String>>,
        attachments: Option<&[String]>,
        tls: bool,
    ) -> Result<(), MailkitError> {
        warn!("send_mail_with_template is deprecated, use send_template instead");
        (self.0).send_template(
            recipient_email,
            subject,
            template_path,
            template_vars,
            cc,
            attachments,
            tls,
        )
    }
}
