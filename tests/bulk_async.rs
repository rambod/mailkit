use futures::stream::{FuturesUnordered, StreamExt};
use lettre::message::{Message, Mailbox};
use lettre::transport::stub::AsyncStubTransport;
use lettre::AsyncTransport;

#[tokio::test]
async fn bulk_async_futures_run() {
    let mailer = AsyncStubTransport::new_ok();
    let mut futs = FuturesUnordered::new();
    for i in 0..5 {
        let msg = Message::builder()
            .from("sender@example.com".parse::<Mailbox>().unwrap())
            .to("rcpt@example.com".parse::<Mailbox>().unwrap())
            .subject("test")
            .body(format!("body {}", i))
            .unwrap();
        futs.push(mailer.send(msg));
    }

    while let Some(res) = futs.next().await {
        res.expect("send failed");
    }
}
