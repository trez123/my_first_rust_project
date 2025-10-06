type DbPool = Pool<AsyncPgConnection>;
use crate::models::user::{NewUser, User};
use crate::schema::users;
use crate::util::crud::Crud;
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl, pooled_connection::deadpool::Pool};

#[derive(Clone)]
pub struct UserRepository {
    pub pool: DbPool,
}

#[async_trait::async_trait]
impl Crud for UserRepository {
    type Item = User;
    type NewItem = NewUser;
    type Error = String;

    async fn create(&self, new_user: NewUser) -> Result<User, String> {
        let mut conn = self.pool.get().await.map_err(|e| e.to_string())?;
        diesel::insert_into(users::table)
            .values(&new_user)
            .returning(User::as_returning())
            .get_result(&mut conn)
            .await
            .map_err(|e| e.to_string())
    }
    async fn read(&self, id: i32) -> Result<User, String> {
        let mut conn = self.pool.get().await.map_err(|e| e.to_string())?;
        users::table
            .find(id)
            .select(User::as_select())
            .first(&mut conn)
            .await
            .map_err(|e| e.to_string())
    }
    async fn delete(&self, id: i32) -> Result<usize, String> {
        let mut conn = self.pool.get().await.map_err(|e| e.to_string())?;
        diesel::delete(users::table.find(id))
            .execute(&mut conn)
            .await
            .map_err(|e| e.to_string())
    }
    async fn list(&self) -> Result<Vec<User>, String> {
        let mut conn = self.pool.get().await.map_err(|e| e.to_string())?;
        users::table
            .select(User::as_select())
            .load(&mut conn)
            .await
            .map_err(|e| e.to_string())
    }
}
