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

#[derive(Deserialize)]
struct OpenIdConfiguration {
    jwks_uri: String,
}

#[derive(Deserialize)]
struct CertsX5CResponse {
    r#use: String,
    x5c: Vec<String>,
}

#[derive(Deserialize)]
struct CertsResponse {
    keys: Vec<CertsX5CResponse>,
}

/// Workaround for  https://github.com/Keats/jsonwebtoken/issues/252 not handling RSA-OAEP
async fn cert_loader(issuer: &str) -> Result<String> {
    debug!("Loading certificates from {}", issuer);

    let mut url =
        reqwest::Url::parse(issuer).with_context(|| format!("Invalid issuer {issuer}"))?;

    url.path_segments_mut()
        .map_err(|_| color_eyre::eyre::eyre!("Issuer URL error! ('{issuer}' cannot be a base)"))?
        .pop_if_empty()
        .extend(&[".well-known", "openid-configuration"]);

    let discovery_endpoint = url.to_string();

    let openid_configuration = reqwest::get(&discovery_endpoint)
        .await
        .map_err(|e| {
            color_eyre::eyre::eyre!(
                "Endpoint {} could not be loaded: {:?}",
                discovery_endpoint,
                e
            )
        })?
        .json::<OpenIdConfiguration>()
        .await
        .map_err(|e| {
            color_eyre::eyre::eyre!(
                "Could not parse response from {}: {:?}",
                discovery_endpoint,
                e
            )
        })?;
    let certs_uri = openid_configuration.jwks_uri;
    let certs_response = reqwest::get(&certs_uri)
        .await
        .map_err(|e| {
            color_eyre::eyre::eyre!(
                "Certificates could not be loaded from {}: {:?}",
                certs_uri,
                e
            )
        })?
        .json::<CertsResponse>()
        .await
        .map_err(|e| {
            color_eyre::eyre::eyre!("Could not parse response from {}: {:?}", certs_uri, e)
        })?;
    certs_response
        .keys
        .iter()
        .find_map(|f| {
            if f.r#use == "sig" {
                Some(format!(
                    "-----BEGIN CERTIFICATE-----\n{}\n-----END CERTIFICATE-----\n",
                    f.x5c[0]
                ))
            } else {
                None
            }
        })
        .ok_or_else(|| color_eyre::eyre::eyre!("No verification key provided"))
}

pub async fn auth_layer(
    issuer: &str,
) -> Result<jwt_authorizer::layer::AsyncAuthorizationLayer<User>> {
    debug!("Using issuer {issuer}");
    let pem_text = cert_loader(&issuer).await?;
    let validation = Validation::new().iss(&[issuer]).leeway(5);
    let jwt_auth = JwtAuthorizer::from_rsa_pem_text(pem_text.as_str()).validation(validation);

    jwt_auth
        .layer()
        .await
        .with_context(|| format!("Invalid JWT configuration for issuer {}", issuer))
}
