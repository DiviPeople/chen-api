use crate::config::AppConfig;
use crate::serializers::TokenClaims;
use actix_web::error::ErrorUnauthorized;
use actix_web::{dev::Payload, Error as ActixWebError};
use actix_web::{http, FromRequest, HttpMessage, HttpRequest};
use jsonwebtoken::{decode, DecodingKey, Validation};
use std::future::{ready, Ready};
use uuid::Uuid;

pub struct JwtMiddleware {
    pub user_id: Uuid,
}

impl FromRequest for JwtMiddleware {
    type Error = ActixWebError;
    type Future = Ready<Result<Self, Self::Error>>;
    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let token = req
            .cookie("token")
            .map(|c| c.value().to_string())
            .or_else(|| {
                req.headers()
                    .get(http::header::AUTHORIZATION)
                    .map(|h| h.to_str().unwrap().split_at(7).1.to_string())
            });

        if token.is_none() {
            return ready(Err(ErrorUnauthorized("You are not logged in")));
        }

        let key = AppConfig::from_env();

        let claims = match decode::<TokenClaims>(
            &token.unwrap(),
            &DecodingKey::from_secret(key.jwt_secret.as_ref()),
            &Validation::default(),
        ) {
            Ok(c) => c.claims,
            Err(_) => {
                return ready(Err(ErrorUnauthorized("You are not logged in")));
            }
        };

        let user_id = claims.sub;
        req.extensions_mut().insert(user_id.to_owned());

        ready(Ok(JwtMiddleware { user_id }))
    }
}
