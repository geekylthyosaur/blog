use actix_session::Session;
use actix_web::{
    web::{Data, Json},
    HttpResponse,
};
use secrecy::Secret;
use serde::Deserialize;
use sqlx::PgPool;

use super::{
    auth::{validate_credentials, AuthError},
    model::Credentials,
};

#[derive(Deserialize, Debug)]
pub struct LoginForm {
    email: String,
    password: Secret<String>,
}

impl From<LoginForm> for Credentials {
    fn from(f: LoginForm) -> Self {
        Self {
            email: f.email,
            password: f.password,
        }
    }
}

#[tracing::instrument(
    name = "Logging in",
    skip(form, pool, session),
    fields(
        email = %form.email,
    )
)]
pub async fn login(
    pool: Data<PgPool>,
    form: Json<LoginForm>,
    session: Session,
) -> Result<HttpResponse, AuthError> {
    let credentials: Credentials = form.into_inner().into();
    let uuid = validate_credentials(&pool, credentials).await?;
    session.insert("user_uuid", uuid).map_err(|e| {
        AuthError::Unexpected(Box::new(e))
    })?;
    Ok(HttpResponse::Ok().finish())
}
