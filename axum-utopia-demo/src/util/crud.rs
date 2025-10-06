#[async_trait::async_trait]
pub trait Crud {
    type Item;
    type NewItem;
    type Error;

    async fn create(&self, item: Self::NewItem) -> Result<Self::Item, Self::Error>;
    async fn read(&self, id: i32) -> Result<Self::Item, Self::Error>;
    async fn delete(&self, id: i32) -> Result<usize, Self::Error>;
    async fn list(&self) -> Result<Vec<Self::Item>, Self::Error>;
}
