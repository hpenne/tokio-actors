use crate::interfaces::ControlA;
use crate::{control_a_marshalling, ControlAMsgs};
use async_trait::async_trait;
use tokio::sync::mpsc::Receiver;

pub struct CompA {}

/// ToDo: Shutdown (task should end on Drop).
impl CompA {
    pub fn new(mut rx: Receiver<ControlAMsgs>) -> Self {
        tokio::spawn(async move {
            // ToDo: Should we instead let CompA hold CompAImpl as an Arc<Mutex<>>?
            // That will public sync methods outside the channel-protocols for setup etc.
            let mut inner = CompAImpl {};
            while let Some(message) = rx.recv().await {
                control_a_marshalling(&message, &mut inner).await;
            }
        });
        Self {}
    }
}

pub struct CompAImpl {}

#[async_trait]
impl ControlA for CompAImpl {
    async fn say_hello(&mut self) {
        print!("Hello ");
    }

    async fn say_world(&mut self) {
        print!("world!");
    }
}
