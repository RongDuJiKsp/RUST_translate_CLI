pub trait AsyncDroppable {
    async fn async_drop(&mut self) -> anyhow::Result<()>;
}
pub trait Closable {
    fn close(self);
}
pub trait AsyncClose {
    async fn async_close(self) -> anyhow::Result<()>;
}