use async_trait::async_trait;

#[async_trait]
pub trait ControlA {
    /// ToDo: Does this need to be async?
    async fn say_hello(&mut self);
    async fn say_world(&mut self);
}
