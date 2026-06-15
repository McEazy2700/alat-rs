//! The HTTP client that drives every ALAT API call.
//!
//! [`Client`] owns a pooled, thread-safe `reqwest::Client` plus the [`Config`]
//! and the **active subscription key**. The endpoint methods live in
//! [`crate::modules`] as inherent methods on `Client`; the low-level
//! [`get_json`](Client::get_json) / [`post_json`](Client::post_json) helpers
//! below do the shared work of URL building, header injection, sending, and
//! decoding.
//!
//! # Multiple products / subscription keys
//!
//! Azure APIM issues a **separate subscription key per product**, and one key
//! only authorizes the APIs inside that product. The SDK's endpoints span
//! several products (e.g. on Playground, "Wallet Services" covers wallet/account/
//! bills/airtime, while statements are a *separate* product with their own key).
//!
//! Rather than force you to build a whole new client per key, the subscription
//! key is injected **per request**, and [`with_subscription_key`](Client::with_subscription_key)
//! cheaply derives a sibling client that **shares the same connection pool**:
//!
//! ```no_run
//! use alat::{Client, Config};
//! # fn main() -> Result<(), alat::Error> {
//! // One client for the "Wallet Services" product...
//! let wallet = Client::new(Config::playground("WALLET_SERVICES_KEY", "api_key"))?;
//! // ...and a sibling for "Get Statement Service" sharing the HTTP pool.
//! let statements = wallet.with_subscription_key("GET_STATEMENT_KEY")?;
//! # let _ = (wallet, statements);
//! # Ok(())
//! # }
//! ```
//!
//! The channel API key (`x-api-key`) is a single per-channel credential issued by
//! Wema and is the same across products, so it lives on [`Config`] and does not
//! need swapping.

use crate::config::Config;
use crate::error::{Error, Result};
use reqwest::header::{HeaderMap, HeaderValue};
use serde::{de::DeserializeOwned, Serialize};
use std::sync::Arc;

/// How many bytes of a failed-to-decode body to keep in [`Error::Decode`].
const BODY_SNIPPET_LIMIT: usize = 2_000;

/// HTTP header carrying the APIM subscription key.
const SUBSCRIPTION_KEY_HEADER: &str = "Ocp-Apim-Subscription-Key";
/// HTTP header carrying the channel API key.
const API_KEY_HEADER: &str = "x-api-key";

/// A configured, reusable ALAT API client.
///
/// Cloning is cheap (the inner `reqwest::Client`, [`Config`], and subscription
/// key are reference counted) and clones share the same connection pool, so a
/// single `Client` can be shared freely across tasks/threads.
///
/// # Example
///
/// ```no_run
/// use alat::{Client, Config};
///
/// # fn main() -> Result<(), alat::Error> {
/// let config = Config::playground("subscription_key", "api_key");
/// let client = Client::new(config)?;
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone)]
pub struct Client {
    config: Arc<Config>,
    /// The subscription key sent on every request from *this* client. Defaults
    /// to [`Config::subscription_key`] but can be swapped per product via
    /// [`with_subscription_key`](Client::with_subscription_key).
    subscription_key: Arc<str>,
    http: reqwest::Client,
}

impl Client {
    /// Builds a client from a [`Config`].
    ///
    /// `Content-Type: application/json` and `Cache-Control: no-cache` (banking
    /// reads must never be cached) are installed as default headers. The
    /// subscription key (`Ocp-Apim-Subscription-Key`) and channel API key
    /// (`x-api-key`) are validated here and then injected on every request.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Configuration`] if a credential contains bytes that are
    /// not valid in an HTTP header value, or if the underlying HTTP client fails
    /// to build. A bad key is reported here, never silently dropped.
    pub fn new(config: Config) -> Result<Self> {
        validate_header_value("subscription_key", &config.subscription_key)?;
        validate_header_value("api_key", &config.api_key)?;
        if let Some(access) = &config.access_key {
            validate_header_value("access_key", access)?;
        }

        let mut headers = HeaderMap::new();
        headers.insert(
            reqwest::header::CONTENT_TYPE,
            HeaderValue::from_static("application/json"),
        );
        headers.insert(
            reqwest::header::CACHE_CONTROL,
            HeaderValue::from_static("no-cache"),
        );

        let http = reqwest::Client::builder()
            .default_headers(headers)
            .timeout(config.timeout)
            .build()
            .map_err(|e| Error::Configuration(format!("failed to build HTTP client: {e}")))?;

        let subscription_key = Arc::from(config.subscription_key.as_str());
        Ok(Self {
            config: Arc::new(config),
            subscription_key,
            http,
        })
    }

    /// Derives a sibling client that uses a **different subscription key** while
    /// sharing this client's connection pool, channel API key, gateway, and
    /// timeout.
    ///
    /// Use this when your endpoints span multiple APIM products (each product
    /// has its own subscription key). See the [module docs](self).
    ///
    /// # Errors
    /// [`Error::Configuration`] if the key has invalid header bytes.
    pub fn with_subscription_key(&self, subscription_key: impl AsRef<str>) -> Result<Self> {
        let key = subscription_key.as_ref();
        validate_header_value("subscription_key", key)?;
        Ok(Self {
            config: self.config.clone(),
            subscription_key: Arc::from(key),
            http: self.http.clone(),
        })
    }

    /// Borrows the [`Config`] this client was built with.
    pub fn config(&self) -> &Config {
        &self.config
    }

    /// The subscription key this client currently sends.
    pub fn subscription_key(&self) -> &str {
        &self.subscription_key
    }

    /// Borrows the underlying `reqwest::Client` for advanced/custom requests.
    pub fn http_client(&self) -> &reqwest::Client {
        &self.http
    }

    /// Joins the configured gateway base URL with a relative API `path`.
    fn url(&self, path: &str) -> String {
        format!(
            "{}/{}",
            self.config.base_url.trim_end_matches('/'),
            path.trim_start_matches('/')
        )
    }

    /// The authentication headers injected on every request: the active
    /// subscription key and the channel API key.
    fn auth_headers(&self) -> Vec<(&'static str, String)> {
        vec![
            (SUBSCRIPTION_KEY_HEADER, self.subscription_key.to_string()),
            (API_KEY_HEADER, self.config.api_key.clone()),
        ]
    }

    /// Issues a `GET` and decodes the JSON response into `T`.
    ///
    /// `query` is a slice of `(name, value)` pairs that are percent-encoded and
    /// appended to the URL. `extra_headers` adds per-request headers (e.g. the
    /// bills/airtime `access` header) on top of the authentication headers.
    pub async fn get_json<T>(
        &self,
        path: &str,
        query: &[(&str, &str)],
        extra_headers: &[(&'static str, String)],
    ) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let mut req = self.http.get(self.url(path));
        if !query.is_empty() {
            req = req.query(query);
        }
        req = self.apply_all_headers(req, extra_headers);
        self.send(req).await
    }

    /// Serializes `body` as JSON, issues a `POST`, and decodes the response into `T`.
    ///
    /// `extra_headers` adds per-request headers (e.g. the funds-transfer `hash`
    /// signature, or the bills/airtime `access` key) alongside auth headers.
    pub async fn post_json<B, T>(
        &self,
        path: &str,
        body: &B,
        extra_headers: &[(&'static str, String)],
    ) -> Result<T>
    where
        B: Serialize + ?Sized,
        T: DeserializeOwned,
    {
        let req = self.apply_all_headers(self.http.post(self.url(path)).json(body), extra_headers);
        self.send(req).await
    }

    /// Applies authentication headers followed by any per-call headers.
    fn apply_all_headers(
        &self,
        mut req: reqwest::RequestBuilder,
        extra_headers: &[(&'static str, String)],
    ) -> reqwest::RequestBuilder {
        for (name, value) in self.auth_headers() {
            req = req.header(name, value);
        }
        for (name, value) in extra_headers {
            req = req.header(*name, value);
        }
        req
    }

    /// Sends a prepared request and routes the outcome through a single, uniform
    /// decode/error path so every endpoint reports failures identically.
    async fn send<T>(&self, req: reqwest::RequestBuilder) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let response = req.send().await?;
        let status = response.status();
        // Read the body as text first so a decode failure can include it for
        // diagnostics instead of being swallowed by `reqwest::Response::json`.
        let body = response.text().await?;

        if !status.is_success() {
            return Err(Error::Http { status, body });
        }

        serde_json::from_str::<T>(&body).map_err(|e| Error::Decode {
            message: e.to_string(),
            body: truncate(&body),
        })
    }

    /// Builds the `access` header list bills/airtime endpoints need.
    ///
    /// `required` controls whether a missing [`Config::access_key`] is a hard
    /// error (for endpoints that mandate it) or simply omitted (for endpoints
    /// where it is optional).
    pub(crate) fn access_headers(&self, required: bool) -> Result<Vec<(&'static str, String)>> {
        match &self.config.access_key {
            Some(key) => Ok(vec![("access", key.clone())]),
            None if required => Err(Error::Configuration(
                "this endpoint requires the bills/airtime `access` key; set it via \
                 Config::with_access_key(...)"
                    .into(),
            )),
            None => Ok(Vec::new()),
        }
    }
}

/// Fails fast if a credential cannot be used as an HTTP header value.
fn validate_header_value(name: &str, value: &str) -> Result<()> {
    HeaderValue::from_str(value)
        .map(|_| ())
        .map_err(|_| Error::Configuration(format!("{name} contains invalid header characters")))
}

/// Truncates a body snippet for inclusion in [`Error::Decode`].
fn truncate(s: &str) -> String {
    if s.len() <= BODY_SNIPPET_LIMIT {
        s.to_string()
    } else {
        format!("{}… [truncated, {} bytes total]", &s[..BODY_SNIPPET_LIMIT], s.len())
    }
}
