mod comp_a;
mod interfaces;
mod marshalling;
mod messages;

use crate::interfaces::ControlA;
use crate::marshalling::connection;
use crate::messages::ControlAMsgs;
use comp_a::CompA;
use marshalling::control_a_server_marshalling;
use std::time;

// ToDo: Return values

#[tokio::main]
async fn main() {
    let (mut client_port, server_rx) = connection::<ControlAMsgs>();
    let _a = CompA::new(rx);

    client_port.say_hello().await;
    let count = client.say_hello().await;
    client_port.say_world().await;
    tokio::time::sleep(time::Duration::from_millis(500)).await;
    println!("Count is {}", count);
}
