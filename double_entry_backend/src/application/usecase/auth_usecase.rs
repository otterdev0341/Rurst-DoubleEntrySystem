use std::sync::Arc;

use thiserror::Error;

use crate::{domain::{dto::{auth_dto::{ReqCreateUserDto, ReqSignInDto, ResSignInDto}, std_201::ResCreateSuccess}, repository::require_implementation::trait_auth::AuthRepoReqImpl}, infrastructure::jwt_service::jwt::generate_jwt};






pub struct AuthUseCase<T>
where
    T: AuthRepoReqImpl + Send + Sync,
{
    auth_repo: Arc<T>,
}


#[derive(Debug, Error)]
pub enum AuthUseCaseError {
    #[error("User not found")]
    UserNotFound,

    #[error("Email already exists")]
    EmailAlreadyExists,

    #[error("Invalid password")]
    InvalidPassword,

    #[error("Internal server error: {0}")]
    InternalError(String),

    #[error("User already exists")]
    UserAlreadyExists,
    
    #[error("Email or password not correct")]
    EmailOrPasswordNotCorrect,

    #[error("Email not fount")]
    EmailNotFound,

    #[error("Hash operation failed")]
    HashFailed,

    #[error("Uuid cast error")]
    UuidCastError
}


impl<T> AuthUseCase<T>
where
    T: AuthRepoReqImpl + Send + Sync,
{
    pub fn new(auth_repo: Arc<T>) -> Self {
        AuthUseCase { auth_repo }
    }

    pub async fn create_user(&self, user_data: ReqCreateUserDto) -> Result<ResCreateSuccess, AuthUseCaseError> {
        match self.auth_repo.create_user(user_data).await {
            Ok(data) => Ok(data),
            Err(e) => Err(e.into()),
        }
    }


    pub async fn sign_in(&self, user_data: ReqSignInDto) -> Result<ResSignInDto, AuthUseCaseError>{

        let user = self.auth_repo.sign_in(user_data).await;

        match user {
            Ok(user) => {
                let jwt = generate_jwt(user.id.to_string(), &user.username);
                match jwt {
                    Ok(token) => Ok(token),
                    Err(_) => Err(AuthUseCaseError::InternalError("jwt generation failed".to_string()))
                }
            },
            Err(e) => Err(e.into())
        }


    }
}