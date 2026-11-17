use axum::extract::Request;

/// Why the gateway does not serve a request the way the Python proxy would.
#[derive(Debug, thiserror::Error, strum::IntoStaticStr)]
pub enum Refused {
    #[error("the gateway has not read the proxy's configuration")]
    #[strum(serialize = "configuration")]
    Configuration,
}

/// Admits the requests the gateway serves the way the Python proxy would, and refuses every other
/// one, giving it back as the client sent it, with why.
// TODO(#52003): admit by the allowlist instead of refusing every request
pub async fn admit(request: Request) -> Result<Request, (Box<Request>, Refused)> {
    Err((Box::new(request), Refused::Configuration))
}
