use color_eyre::{eyre::Context, Result};
use jwt_authorizer::{JwtAuthorizer, Validation};
use serde::Deserialize;
use tracing::debug;

#[derive(Debug, Deserialize, Clone)]
pub(crate) struct RealmAccess {
    roles: Vec<String>,
}

// struct representing the authorized caller, deserializable from JWT claims
#[derive(Debug, Deserialize, Clone)]
pub struct User {
    sub: String,
    preferred_username: String,
    realm_access: RealmAccess,
}

impl User {
    pub fn preferred_username(&self) -> &str {
        self.preferred_username.as_ref()
    }
}

pub async fn auth_layer(issuer: &str) -> Result<jwt_authorizer::layer::AuthorizationLayer<User>> {
    debug!("Using issuer {issuer}");
    let validation = Validation::new().iss(&[issuer]).leeway(5);
    let jwt_auth = JwtAuthorizer::from_oidc(issuer).validation(validation);

    jwt_auth
        .layer()
        .await
        .with_context(|| format!("Invalid JWT configuration for issuer {}", issuer))
}
