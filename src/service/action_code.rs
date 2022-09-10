use std::fmt::Debug;
use tokio::sync::{mpsc, oneshot};

pub fn start_service<T>(buffer: usize, success_sig: oneshot::Sender<mpsc::Sender<T>>) where
    T: Debug + Send + 'static
{
    tokio::spawn(async move {
        let (sender, mut channel) = mpsc::channel(buffer);
        if let Err(_) = success_sig.send(sender) {
            tracing::warn!("Action code service starter is dropped! Cancel the start process.");
            return;
        }

        while let Some(cmd) = channel.recv().await {
            println!("{:?}", cmd);
        }
    });
}

#[derive(Debug)]
pub enum Command { HelloWorld }

pub async fn test_send() -> Option<mpsc::Sender<Command>> {
    let (tx, rx) = oneshot::channel();
    start_service(32, tx);

    let mut result = None;

    match rx.await {
        Ok(sender) => { result = Some(sender.clone()); sender.send(Command::HelloWorld).await.unwrap(); }
        Err(err) => { tracing::error!("Fail to start service with {}", err); }
    };

    result
}
