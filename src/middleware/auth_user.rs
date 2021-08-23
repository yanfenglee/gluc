use actix_web::{FromRequest, HttpRequest};
use actix_http::{Payload, Error};
use actix_http::error::ParseError;
use actix_http::http::HeaderMap;
use rbatis::sql;
use std::pin::Pin;
use futures::{Future};

use crate::util::local_cache::CACHE_I64;
use crate::dao::RB;

#[derive(Debug)]
pub struct AuthUser {
    pub(crate) user_id: i64,
    pub(crate) token: String,
}

impl AuthUser {
    pub async fn from_header(headers: &HeaderMap) -> Option<Self> {
        if let Some(header) = headers.get("api-secret") {
            let token = header.to_str().unwrap().to_string();

            // get from local cache first
            if let Ok(cc) = CACHE_I64.lock() {
                if let Some(id) = cc.get(&token) {
                    return Some(AuthUser { user_id: *id, token });
                }
            }

            // get from db
            #[sql(RB, "SELECT id FROM user WHERE token = #{token} LIMIT 1")]
            async fn select_id(token: &String) -> Option<i64> {}

            if let Ok(Some(id)) = select_id(&token).await {

                // write cache
                if let Ok(mut cc) = CACHE_I64.lock() {
                    cc.insert(token.clone(), id);
                }

                log::info!("cache miss, get from db: {}", id);

                return Some(AuthUser { user_id: id, token: token.clone() });
            }
        }

        None
    }
}

impl FromRequest for AuthUser {
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output=Result<Self, Self::Error>>>>;
    type Config = ();

    fn from_request(
        req: &HttpRequest,
        _payload: &mut Payload,
    ) -> Self::Future {
        let headers = req.headers().clone();

        let ret = async move {
            if let Some(user) = AuthUser::from_header(&headers).await {
                Ok(user)
            } else {
                Err(Error::from(ParseError::Header))
            }
        };

        Box::pin(ret)
    }
}