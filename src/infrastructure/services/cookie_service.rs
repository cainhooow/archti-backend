use std::env;

use salvo::{
    Response,
    http::{
        cookie::{Cookie, CookieBuilder, Expiration, SameSite},
        header::REFRESH,
    },
};
use time::{Duration, OffsetDateTime};

use crate::application::ports::token_service::TokenOutput;

#[derive(Default, Debug)]
pub struct CookieService {}

pub const COOKIE_SESSION_NAME: &'static str = "session";
pub const COOKIE_REFRESH_NAME: &'static str = "refresh";

impl CookieService {
    pub fn new() -> Self {
        Self {}
    }

    pub fn session_cookie<'c>(
        &'c self,
        name: impl Into<String>,
        value: impl Into<String>,
        duration: Expiration,
    ) -> Cookie<'static> {
        let app_mode = env::var("APP_ENV").expect("APP_ENV is not defined in .env");

        let cookie_builder = CookieBuilder::new(name.into(), value.into())
            .http_only(true)
            .secure(app_mode.eq("production"))
            .same_site(SameSite::Strict)
            .path("/")
            .expires(duration);

        let cookie = cookie_builder.build();
        cookie
    }

    pub fn generate_sessions(
        &self,
        access: &TokenOutput,
        refresh: &TokenOutput,
        res: &mut Response,
    ) {
        let access_expiry = OffsetDateTime::from_unix_timestamp(access.expires_at)
            .unwrap_or_else(|_| OffsetDateTime::now_utc() + Duration::minutes(15));

        let refresh_expiry = OffsetDateTime::from_unix_timestamp(refresh.expires_at)
            .unwrap_or_else(|_| OffsetDateTime::now_utc() + Duration::days(7));

        let access_token = self.session_cookie(
            COOKIE_SESSION_NAME.to_string(),
            &access.token,
            Expiration::DateTime(access_expiry),
        );

        let refresh_cookie = self.session_cookie(
            COOKIE_REFRESH_NAME.to_string(),
            &refresh.token,
            Expiration::DateTime(refresh_expiry),
        );

        res.add_cookie(access_token);
        res.add_cookie(refresh_cookie);
    }
}
