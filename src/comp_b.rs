use crate::interfaces::HelloEvent;
use crate::HelloEvent::HelloFrom;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};

pub struct CompB {}

impl CompB {
    pub fn new(
        mut rx: UnboundedReceiver<HelloEvent>,
        hello_tx: UnboundedSender<HelloEvent>,
    ) -> Self {
        tokio::spawn(async move {
            // That will public sync methods outside the channel-protocols for setup etc.
            let mut inner = CompBImpl { hello_tx };
            while let Some(message) = rx.recv().await {
                match message {
                    HelloFrom { sender } => {
                        inner.hello_from(sender);
                    }
                }
            }
        });
        Self {}
    }
}

pub struct CompBImpl {
    hello_tx: UnboundedSender<HelloEvent>,
}

impl CompBImpl {
    fn hello_from(&mut self, sender: String) {
        println!("B: Hello from {}", sender);
        self.hello_tx
            .send(HelloFrom {
                sender: "B".to_owned(),
            })
            .expect("Actor is gone");
    }
}
