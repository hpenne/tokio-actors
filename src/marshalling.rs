use crate::interfaces::{ControlA, HelloEvent};
use crate::messages::{ControlAMsgs, HelloEventMsgs};
use async_trait::async_trait;
use tokio::sync::mpsc;
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::sync::oneshot;

pub fn connection<T>() -> (ClientMarshaller<T>, Receiver<T>) {
    let (tx, rx) = mpsc::channel::<T>(32);
    (ClientMarshaller::<T> { tx }, rx)
}

pub struct ClientMarshaller<T> {
    tx: Sender<T>,
}

/// This could be generated
#[async_trait]
impl ControlA for ClientMarshaller<ControlAMsgs> {
    async fn say_hello(&mut self) -> usize {
        let (response_tx, response_rx) = oneshot::channel::<usize>();

        // Both of these will fail if the server is gone, so we only handle the error once:
        let _ = self.tx.send(ControlAMsgs::SayHello { response_tx }).await;
        response_rx.await.expect("Actor is gone")
    }

    async fn say_world(&mut self) {
        let _ = self.tx.send(ControlAMsgs::SayWorld {}).await;
    }
}

/// This could be generated
pub async fn control_a_server_marshalling<T: ControlA>(msg: ControlAMsgs, target: &mut T) {
    match msg {
        ControlAMsgs::SayHello { response_tx } => {
            response_tx.send(target.say_hello().await).unwrap();
        }
        ControlAMsgs::SayWorld => {
            target.say_world().await;
        }
    }
}

/// This could be generated
#[async_trait]
impl HelloEvent for ClientMarshaller<HelloEventMsgs> {
    async fn hello_from(&mut self, sender: String) {
        let _ = self.tx.send(HelloEventMsgs::HelloFrom { sender }).await;
    }
}

/// This could be generated
pub async fn hello_event_server_marshalling<T: HelloEvent>(msg: HelloEventMsgs, target: &mut T) {
    match msg {
        HelloEventMsgs::HelloFrom { sender } => {
            target.hello_from(sender).await;
        }
    }
}
