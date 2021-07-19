use crate::generated::{
    components::{request_bodies::AuthUrlRequestBody, responses::AuthUrlSuccess},
    paths::auth_url::{Error, Response},
};
use actix_swagger::{Answer, ContentType};
use actix_web::http::StatusCode;
use actix_web::web::{Data, Json};
use cardbox_settings::Settings;
use eyre::WrapErr;
use serde::{Deserialize, Serialize};
use url::Url;

pub async fn route(
    body: Json<AuthUrlRequestBody>,
    config: Data<Settings>,
) -> Result<Response, Error> {
    let mut accesso = Url::parse(&config.accesso.url).wrap_err("Could not parse url")?;

    accesso.set_path("/oauth/authorize");

    {
        let mut pairs = accesso.query_pairs_mut();
        pairs
            .append_pair("response_type", "code")
            .append_pair("redirect_uri", &config.accesso.redirect_back_url)
            .append_pair("client_id", &config.accesso.client_id)
            .append_pair("state", &body.state);
    }

    let response = Response::Ok(AuthUrlSuccess {
        accesso_url: accesso.to_string(),
    });

    Ok(response)
}
