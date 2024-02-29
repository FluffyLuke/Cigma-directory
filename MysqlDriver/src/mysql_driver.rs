use async_trait::async_trait;
use sqlx::{MySql, Pool};
use DataBridge::{db_connector::{DatabaseDriverError, GenericDatabase, QueryResult}, models::{self, WeakUser}};

pub struct MysqlDatabase {
    pool: Pool<MySql>
}

#[async_trait]
impl GenericDatabase for MysqlDatabase {
    async fn get_user(&self, weak_user: &WeakUser) -> Result<models::User, DatabaseDriverError> {
        let user = sqlx::query_as!(
            models::User,
            "SELECT user_id, name, last_name, email, nickname FROM users WHERE email = ? AND nickname = ?",
            weak_user.email, weak_user.nickname
        ).fetch_one(&self.pool)
        .await
        .map_err(|err| DatabaseDriverError::BasicError(err))?;
        return Ok(user)
    }

    async fn add_user(&self, user: DataBridge::models::User, password: DataBridge::models::Password) -> QueryResult<()> {
        let mut transaction = self.pool
            .begin()
            .await
            .map_err(|err| DatabaseDriverError::BasicError(err))?;

        sqlx::query!("INSERT INTO users (name, last_name, email, nickname) VALUES (?, ?, ?, ?)",
            user.name, user.last_name, user.email, user.nickname)
            .execute(&mut *transaction)
            .await
            .map_err(|err| DatabaseDriverError::BasicError(err))?;

        sqlx::query!("INSERT INTO passwords (user_id, password) VALUES (?, ?)",
            user.user_id, password.password)
            .execute(&mut *transaction)
            .await
            .map_err(|err| DatabaseDriverError::BasicError(err))?;

        transaction.commit().await.map_err(|err| DatabaseDriverError::BasicError(err))?;
        Ok(())
    }

    async fn remove_user(&self, user: DataBridge::models::User, password: DataBridge::models::Password) -> QueryResult<()> {
        let mut transaction = self.pool
            .begin()
            .await
            .map_err(|err| DatabaseDriverError::BasicError(err))?;

        sqlx::query!("INSERT INTO users (name, last_name, email, nickname) VALUES (?, ?, ?, ?)",
            user.name, user.last_name, user.email, user.nickname)
            .execute(&mut *transaction)
            .await
            .map_err(|err| DatabaseDriverError::BasicError(err))?;

        sqlx::query!("INSERT INTO passwords (user_id, password) VALUES (?, ?)",
            user.user_id, password.password)
            .execute(&mut *transaction)
            .await
            .map_err(|err| DatabaseDriverError::BasicError(err))?;

        transaction.commit().await.map_err(|err| DatabaseDriverError::BasicError(err))?;
        Ok(())
    }

    async fn change_password(
        &self, 
        user: DataBridge::models::User, 
        old_password: DataBridge::models::Password, 
        new_password: DataBridge::models::Password
    ) -> QueryResult<()> {
        todo!()
    }

    async fn add_service(service: DataBridge::models::RegisteredService) -> QueryResult<()> {
        todo!()
    }

    async fn add_user_account(
        &self, 
        user: DataBridge::models::User, 
        service: DataBridge::models::RegisteredService,
        conf: String
    ) -> QueryResult<()> {
        todo!()
    }

    async fn verify_user(&self, verification_type: DataBridge::db_connector::VerificationType) -> QueryResult<bool> {
        todo!()
    }
}