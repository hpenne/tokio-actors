mod comp_a;
mod comp_b;
mod interfaces;

use crate::comp_b::CompB;
use crate::ControlA::{SayHello, SayWorld};
use comp_a::CompA;
use interfaces::{ControlA, HelloEvent};
use std::time;
use tokio::sync::mpsc;
use tokio::sync::mpsc::UnboundedSender;
use tokio::sync::oneshot;

async fn say_hello(client: &UnboundedSender<ControlA>) -> usize {
    let (response_tx, response_rx) = oneshot::channel::<usize>();
    client.send(SayHello { response_tx }).unwrap();
    response_rx.await.unwrap()
}

#[tokio::main]
async fn main() {
    let (a_hello_tx, b_hello_rx) = mpsc::unbounded_channel::<HelloEvent>();
    let (b_hello_tx, a_hello_rx) = mpsc::unbounded_channel::<HelloEvent>();
    let (ctrl_client_for_a, a_ctrl_server_rx) = mpsc::unbounded_channel::<ControlA>();
    let _a = CompA::new(a_ctrl_server_rx, a_hello_rx, a_hello_tx);
    let _b = CompB::new(b_hello_rx, b_hello_tx);

    say_hello(&ctrl_client_for_a).await;
    let count = say_hello(&ctrl_client_for_a).await;
    ctrl_client_for_a.send(SayWorld).unwrap();

    tokio::time::sleep(time::Duration::from_millis(500)).await;
    println!("Count is {}", count);
}
