use crate::interfaces::HelloEvent;
use crate::marshalling::hello_event_server_marshalling;
use crate::messages::HelloEventMsgs;
use async_trait::async_trait;
use tokio::sync::mpsc::Receiver;

pub struct CompB {}

impl CompB {
    pub fn new(mut rx: Receiver<HelloEventMsgs>) -> Self {
        tokio::spawn(async move {
            // That will public sync methods outside the channel-protocols for setup etc.
            let mut inner = CompBImpl {};
            while let Some(message) = rx.recv().await {
                hello_event_server_marshalling(message, &mut inner).await;
            }
        });
        Self {}
    }
}

pub struct CompBImpl {}

#[async_trait]
impl HelloEvent for CompBImpl {
    async fn hello_from(&mut self, sender: String) {
        println!("Hello from {}", sender);
    }
}
