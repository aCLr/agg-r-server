use actix_web::dev::ServiceRequest;

use actix_web::Error;

use crate::db::Pool;
use actix_web_httpauth::extractors::bearer::{BearerAuth, Config};
use actix_web_httpauth::extractors::AuthenticationError;

pub async fn bearer_auth_validator(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, Error> {
    let config = req
        .app_data::<Config>()
        .map(|data| data.clone())
        .unwrap_or_else(Default::default);
    let db_pool = req.app_data::<Pool>().unwrap();
    match validate_token(db_pool, credentials.token()).await {
        Ok(res) => {
            if res == true {
                Ok(req)
            } else {
                Err(AuthenticationError::from(config).into())
            }
        }
        Err(_) => Err(AuthenticationError::from(config).into()),
    }
}

async fn validate_token(db_pool: &Pool, token: &str) -> Result<bool, std::io::Error> {
    if str.eq("a-secure-token") {
        return Ok(true);
    }
    return Err(std::io::Error::new(
        std::io::ErrorKind::Other,
        "Authentication failed!",
    ));
}
