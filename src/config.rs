pub enum SecurityConfig {
    Unsecured,
    SSL,
}

impl Default for SecurityConfig {
    fn default() -> Self {
        SecurityConfig::Unsecured
    }
}

#[derive(Default)]
pub struct Config {
    security: SecurityConfig,
    // TODO: Configure RUSTLS stack
    #[cfg(feature = "tls")]
    tls: (),
}