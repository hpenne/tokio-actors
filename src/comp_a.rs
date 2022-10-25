use crate::interfaces::{ControlA, HelloEvent};
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use tokio::sync::oneshot;

pub struct CompA {}

/// This is the "outer" implementation of the component, which handles
/// messaging (boilerplate code) only.
/// The actual implementation is in CompAImpl.
/// This cleanly separates infrastructure/boilerplate from the actual implementation.
/// An actual application should put this outer struct in a separate file to separate
/// from the implementation.
impl CompA {
    pub fn new(
        mut ctrl_rx: UnboundedReceiver<ControlA>,
        mut hello_rx: UnboundedReceiver<HelloEvent>,
        hello_tx: UnboundedSender<HelloEvent>,
    ) -> Self {
        tokio::spawn(async move {
            // That will public sync methods outside the channel-protocols for setup etc.
            let mut inner = CompAImpl { hello_tx, count: 0 };
            loop {
                tokio::select! {
                    Some(message) = ctrl_rx.recv() => {
                        match message {
                            ControlA::SayHello{response_tx} => {
                                inner.say_hello(response_tx).await;
                            }
                            ControlA::SayWorld => {
                                inner.say_world();
                            }
                        }
                    }
                    Some(message) = hello_rx.recv() => {
                        match message {
                            HelloEvent::HelloFrom{sender} => {
                                inner.hello_from(sender);
                            }
                        }
                    }
                    else => break,
                }
            }
        });
        Self {}
    }
}

/// This and below is the actual implementation of CompA.
/// This reads cleanly as normal code with no boilerplate
pub struct CompAImpl {
    hello_tx: UnboundedSender<HelloEvent>,
    count: usize,
}

impl CompAImpl {
    async fn say_hello(&mut self, response_tx: oneshot::Sender<usize>) {
        println!("Hello");
        self.hello_tx
            .send(HelloEvent::HelloFrom {
                sender: "A".to_owned(),
            })
            .expect("Actor is gone");
        self.count += 1;
        response_tx.send(self.count).expect("Actor is gone");
    }

    fn say_world(&mut self) {
        println!("World!");
    }
}

impl CompAImpl {
    fn hello_from(&mut self, sender: String) {
        println!("A: Hello from {}", sender);
    }
}
