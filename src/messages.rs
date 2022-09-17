use tokio::sync::oneshot::Sender;

#[derive(Debug)]
pub enum ControlAMsgs {
    SayHello { response_tx: Sender<usize> },
    SayWorld,
}

pub enum HelloEventMsgs {
    HelloFrom { sender: String },
}
