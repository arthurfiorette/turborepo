use reqwest::Url;
use turborepo_auth::{logout as auth_logout, LogoutOptions};
use turborepo_telemetry::events::command::CommandEventBuilder;

use crate::{cli::Error, commands::CommandBase};

pub async fn logout(
    base: &mut CommandBase,
    invalidate: bool,
    _telemetry: CommandEventBuilder,
) -> Result<(), Error> {
    let path = if is_vercel_login_url(base.opts.api_client_opts.login_url.as_str()) {
        None
    } else {
        Some(base.local_config_path())
    };

    auth_logout(&LogoutOptions {
        color_config: base.color_config,
        api_client: base.api_client()?,
        invalidate,
        path,
    })
    .await
    .map_err(Error::from)
}

fn is_vercel_login_url(login_url: &str) -> bool {
    Url::parse(login_url)
        .ok()
        .and_then(|parsed| parsed.host_str().map(str::to_lowercase))
        .is_some_and(|host| host == "vercel.com" || host.ends_with(".vercel.com"))
}
