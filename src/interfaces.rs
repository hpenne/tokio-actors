use async_trait::async_trait;

#[async_trait]
pub trait ControlA {
    async fn say_hello(&mut self) -> usize;
    async fn say_world(&mut self);
}

#[async_trait]
pub trait HelloEvent {
    async fn hello_from(&mut self, sender: String);
}
