//! Client configuration: gateway host, credentials, and transport tuning.
//!
//! # The ALAT gateway topology (important)
//!
//! ALAT by Wema publishes its APIs through **Azure API Management (APIM)**. A
//! single APIM gateway fronts many *products*, each mounted under a path prefix
//! (e.g. `/funds-transfer-open`, `/bills-payment`, `/wallet-creation`). Every
//! request is therefore `https://<gateway-host>/<product-prefix>/...`.
//!
//! Wema exposes **two public developer sandboxes**, and — crucially — they are
//! *separate* APIM instances with *separate* subscription keys and *different*
//! sets of products:
//!
//! | Sandbox       | Sign-up portal                                    | API gateway (calls go here)                 |
//! |---------------|---------------------------------------------------|---------------------------------------------|
//! | **Playground**| `https://playground.alat.ng`                      | `https://playground.azure-api.net`          |
//! | **APIM Dev**  | `https://wema-alatdev-apimgt.developer.azure-api.net` | `https://wema-alatdev-apimgt.azure-api.net` |
//!
//! > The portal host (where you sign up / read keys) is **not** the gateway host
//! > (where API calls go). The named constructors below target the **gateway**.
//!
//! Because a [`Client`](crate::Client) is bound to exactly one gateway + key,
//! exercising endpoints that live on *both* sandboxes requires two clients (see
//! the crate-level docs). In **production**, Wema typically fronts all of your
//! subscribed products behind a single gateway host that they issue to you —
//! pass that host to [`Config::new`].
//!
//! > The previous revision of this SDK defaulted to `https://sandbox.alat.ng`,
//! > which does not resolve. There is intentionally **no hidden default host**
//! > here: you must name the gateway explicitly (or use a named sandbox
//! > constructor), so a typo can never silently point at the wrong bank.

use std::time::Duration;

/// The default per-request timeout applied if the caller does not override it.
///
/// Banking calls should never hang indefinitely; 30s is generous for the
/// synchronous "accepted/pending" responses these endpoints return.
pub const DEFAULT_TIMEOUT: Duration = Duration::from_secs(30);

/// Connection details and credentials for a single ALAT gateway.
///
/// A `Config` is consumed by [`Client::new`](crate::Client::new). It is cheap to
/// clone and holds no live connections.
///
/// # Authentication
///
/// APIM gates every product with a **subscription key**
/// (`Ocp-Apim-Subscription-Key`). On top of that, ALAT identifies the calling
/// software *channel* with an **API key** (`x-api-key`). A subset of products
/// (bills & airtime) additionally require an **access key** sent in an `access`
/// header — supply it via [`Config::with_access_key`] when you use those
/// endpoints.
#[derive(Debug, Clone)]
pub struct Config {
    /// Absolute base URL of the APIM **gateway** (where API calls go, not the
    /// sign-up portal), e.g. `https://playground.azure-api.net`. Stored without a
    /// trailing slash.
    pub base_url: String,

    /// APIM subscription key, sent as the `Ocp-Apim-Subscription-Key` header.
    /// Obtained from your subscription on the ALAT developer portal.
    pub subscription_key: String,

    /// Channel API key, sent as the `x-api-key` header. Identifies your
    /// registered application/partner to ALAT's core systems.
    pub api_key: String,

    /// Optional "access" credential required by the bills & airtime products,
    /// sent as the `access` header. `None` if you do not use those endpoints.
    pub access_key: Option<String>,

    /// Maximum duration to wait for each request before failing with a
    /// [`Network`](crate::Error::Network) timeout error.
    pub timeout: Duration,
}

impl Config {
    /// Creates a configuration pointing at an explicit gateway host.
    ///
    /// This is the honest, production-oriented constructor: you name the gateway
    /// Wema issued to you. Trailing slashes are trimmed.
    ///
    /// ```
    /// use alat::Config;
    /// let config = Config::new("https://your-gateway.wemabank.com", "sub_key", "api_key");
    /// assert_eq!(config.base_url, "https://your-gateway.wemabank.com");
    /// ```
    pub fn new(
        base_url: impl Into<String>,
        subscription_key: impl Into<String>,
        api_key: impl Into<String>,
    ) -> Self {
        let base_url = base_url.into().trim_end_matches('/').to_string();
        Self {
            base_url,
            subscription_key: subscription_key.into(),
            api_key: api_key.into(),
            access_key: None,
            timeout: DEFAULT_TIMEOUT,
        }
    }

    /// Configuration for the **Playground** sandbox gateway
    /// (`https://playground.azure-api.net`; sign up at `https://playground.alat.ng`).
    ///
    /// Use this for wallet creation, account maintenance, statements, bills, and
    /// airtime — the products published on that portal.
    pub fn playground(
        subscription_key: impl Into<String>,
        api_key: impl Into<String>,
    ) -> Self {
        Self::new("https://playground.azure-api.net", subscription_key, api_key)
    }

    /// Configuration for the **APIM Dev** sandbox
    /// (`https://wema-alatdev-apimgt.azure-api.net`).
    ///
    /// Use this for the funds-transfer / name-enquiry product
    /// (`/funds-transfer-open`), which is published on that portal rather than
    /// on Playground.
    pub fn apim_dev(
        subscription_key: impl Into<String>,
        api_key: impl Into<String>,
    ) -> Self {
        Self::new(
            "https://wema-alatdev-apimgt.azure-api.net",
            subscription_key,
            api_key,
        )
    }

    /// Attaches the `access` credential required by bills & airtime endpoints
    /// (builder style).
    ///
    /// ```
    /// use alat::Config;
    /// let config = Config::playground("sub", "api").with_access_key("bills_access_token");
    /// assert!(config.access_key.is_some());
    /// ```
    pub fn with_access_key(mut self, access_key: impl Into<String>) -> Self {
        self.access_key = Some(access_key.into());
        self
    }

    /// Overrides the default per-request [`timeout`](Self::timeout) (builder style).
    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }
}
