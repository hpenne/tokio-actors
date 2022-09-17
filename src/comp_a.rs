use crate::interfaces::ControlA;
use crate::marshalling::ClientMarshaller;
use crate::{control_a_server_marshalling, ControlAMsgs, HelloEvent, HelloEventMsgs};
use async_trait::async_trait;
use tokio::sync::mpsc::Receiver;

pub struct CompA {}

/// ToDo: Shutdown (task should end on Drop).
impl CompA {
    pub fn new(mut rx: Receiver<ControlAMsgs>, b_client: ClientMarshaller<HelloEventMsgs>) -> Self {
        tokio::spawn(async move {
            // ToDo: Should we instead let CompA hold CompAImpl as an Arc<Mutex<>>?
            // That will public sync methods outside the channel-protocols for setup etc.
            let mut inner = CompAImpl { b_client, count: 0 };
            while let Some(message) = rx.recv().await {
                control_a_server_marshalling(message, &mut inner).await;
            }
        });
        Self {}
    }
}

pub struct CompAImpl {
    b_client: ClientMarshaller<HelloEventMsgs>,
    count: usize,
}

#[async_trait]
impl ControlA for CompAImpl {
    async fn say_hello(&mut self) -> usize {
        print!("Hello ");
        self.b_client.hello_from("A".to_owned()).await;
        self.count += 1;
        self.count
    }

    async fn say_world(&mut self) {
        println!("world!");
    }
}
