use diesel_async::pooled_connection::deadpool::{BuildError, Pool};
use diesel_async::{AsyncPgConnection, pooled_connection::AsyncDieselConnectionManager};
use dotenvy::dotenv;
use std::env;

pub type DbPool = Pool<AsyncPgConnection>;

pub async fn create_pool() -> Result<DbPool, BuildError> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let manager = AsyncDieselConnectionManager::<AsyncPgConnection>::new(database_url);
    Pool::builder(manager).build()
}
