use tokio::sync::oneshot::Sender;

#[derive(Debug)]
pub enum ControlAMsgs {
    SayHello { response_tx: Sender<usize> },
    SayWorld,
}

//pub enum EventsAMsgs {
//    HelloFromA,
//}
//
//pub enum EventsBMsgs {
//    HelloFromB,
//}
