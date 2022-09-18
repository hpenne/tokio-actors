mod comp_a;
mod comp_b;
mod interfaces;
mod marshalling;
mod messages;

use crate::comp_b::CompB;
use crate::interfaces::{ControlA, HelloEvent};
use crate::marshalling::connection;
use crate::messages::{ControlAMsgs, HelloEventMsgs};
use comp_a::CompA;
use marshalling::control_a_server_marshalling;
use std::time;

#[tokio::main]
async fn main() {
    let (b_client_port, b_server_rx) = connection::<HelloEventMsgs>();
    let (mut a_client_port, a_server_rx) = connection::<ControlAMsgs>();
    let _b = CompB::new(b_server_rx);
    let _a = CompA::new(a_server_rx, b_client_port);

    a_client_port.say_hello().await;
    let count = a_client_port.say_hello().await;
    a_client_port.say_world();

    tokio::time::sleep(time::Duration::from_millis(500)).await;
    println!("Count is {}", count);
}
