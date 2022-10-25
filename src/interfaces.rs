use tokio::sync::oneshot;

#[derive(Debug)]
pub enum ControlA {
    SayHello { response_tx: oneshot::Sender<usize> },
    SayWorld,
}

#[derive(Debug)]
pub enum HelloEvent {
    HelloFrom { sender: String },
}
