use crate::interfaces::ControlA;
use crate::messages::ControlAMsgs;
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
        let _ = self.tx.send(ControlAMsgs::SayHello { response_tx }).await;
        response_rx.await.unwrap()
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
