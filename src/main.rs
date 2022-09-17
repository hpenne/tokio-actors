mod comp_a;
mod interfaces;
mod marshalling;
mod messages;

use crate::messages::ControlAMsgs;
use comp_a::CompA;
use marshalling::control_a_marshalling;
use std::time;
use tokio::sync::mpsc;

// ToDo: Return values

#[tokio::main]
async fn main() {
    let (tx, rx) = mpsc::channel::<ControlAMsgs>(32);
    let _a = CompA::new(rx);
    tx.send(ControlAMsgs::SayHello {}).await.unwrap();
    tx.send(ControlAMsgs::SayWorld {}).await.unwrap();

    tokio::time::sleep(time::Duration::from_millis(500)).await;
}
