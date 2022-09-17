use crate::interfaces::ControlA;
use crate::messages::ControlAMsgs;

/// This could be generated
pub async fn control_a_marshalling<T: ControlA>(msg: &ControlAMsgs, target: &mut T) {
    match msg {
        ControlAMsgs::SayHello => {
            target.say_hello().await;
        }
        ControlAMsgs::SayWorld => {
            target.say_world().await;
        }
    }
}
