use std::fmt::Display;

use reqwest::Proxy;
use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct ProxyConfig {
    pub url:      String,
    pub username: String,
    pub password: String,
}
impl ProxyConfig {
    pub fn new(url: String, username: String, password: String) -> Self {
        Self {
            url,
            username,
            password,
        }
    }
    pub fn build(&self) -> Proxy {
        let proxy_scheme = self.url.clone();
        Proxy::https(proxy_scheme)
            .unwrap()
            .basic_auth(self.username.as_str(), self.password.as_str())
    }
}

impl Default for ProxyConfig {
    fn default() -> Self {
        ProxyConfig {
            url:      String::new(),
            username: String::new(),
            password: String::new(),
        }
    }
}
