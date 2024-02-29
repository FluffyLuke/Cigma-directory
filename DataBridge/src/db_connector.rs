use crate::models;

pub type QueryResult<T> = Result<T, DatabaseDriverError>;


#[async_trait::async_trait]
pub trait GenericDatabase {
    async fn get_user(&self, weak_user: &models::WeakUser) -> Result<models::User, DatabaseDriverError>;
    async fn add_user(&self, user: models::User, password: models::Password) -> QueryResult<()>;
    async fn remove_user(&self, user: models::User, password: models::Password) -> QueryResult<()>;
    async fn change_password(
        &self, 
        user: models::User, 
        old_password: models::Password, 
        new_password: models::Password
    ) -> QueryResult<()>;
    async fn add_service(service: models::RegisteredService) -> QueryResult<()>;
    async fn add_user_account(
        &self, 
        user: models::User, 
        service: models::RegisteredService,
        conf: String
    ) -> QueryResult<()>;
    async fn verify_user(&self, verification_type: VerificationType) -> QueryResult<bool>;
}

// TODO end this error
pub enum DatabaseDriverError {
    BasicError(sqlx::error::Error),
    UserNotFound,
}

// TODO add later more verification types, like autorization to services
pub enum VerificationType {
    Autentification(models::User, models::Password)
}