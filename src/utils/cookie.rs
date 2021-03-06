use anyhow::Context;
use cookie_store;
use reqwest_cookie_store::CookieStoreMutex;
use serde_json::{Error, Value};
use std::sync::Arc;
use std::{
    fs::{self, File, OpenOptions},
    io::{self, Read},
};

use crate::configs::config::Config;

pub struct CookieStore {
    arc: Arc<CookieStoreMutex>,
}

impl CookieStore {
    async fn path() -> anyhow::Result<String> {
        let conf = Config::load().await.context("failed to load config")?;
        let path = conf
            .cookie
            .as_ref()
            .context("cookie is not set")?
            .path
            .clone();
        Ok(path)
    }
    pub async fn load() -> anyhow::Result<Arc<CookieStoreMutex>> {
        let path = CookieStore::path().await?;
        let is_exist = std::path::Path::new(&path).exists();
        if !is_exist {
            fs::File::create(&path).context("failed to create cookie file")?;
        }
        let reader = File::open(&path)
            .map(io::BufReader::new)
            .context("failed to open cookie file")?;

        let cookie_store = cookie_store::CookieStore::load_json(reader).unwrap();
        let cookie_store = CookieStoreMutex::new(cookie_store);
        let cookie_store = std::sync::Arc::new(cookie_store);

        Ok(cookie_store)
    }

    pub async fn save(arc: Arc<CookieStoreMutex>) -> anyhow::Result<()> {
        let cookie_path = CookieStore::path().await?;
        let mut writer = File::create(&cookie_path)
            .map(io::BufWriter::new)
            .context("failed to create cookie file")?;
        let store = arc.lock().unwrap();
        store.save_json(&mut writer).unwrap();
        Ok(())
    }

    pub async fn clear() -> anyhow::Result<()> {
        let cookie_path = CookieStore::path().await?;
        let is_exist = std::path::Path::new(&cookie_path).exists();
        if is_exist {
            std::fs::remove_file(&cookie_path).context("failed to remove cookie file")?;
        };
        Ok(())
    }

    pub fn arc(&self) -> Arc<CookieStoreMutex> {
        Arc::clone(&self.arc)
    }

    pub async fn load_raw() -> Option<String> {
        let path = CookieStore::path().await.unwrap();
        let file = OpenOptions::new().read(true).open(&path);
        let mut file = match file {
            Ok(file) => file,
            Err(_) => return None,
        };
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        let v: Result<Value, Error> = serde_json::from_str(&contents);
        let v = match v {
            Ok(v) => v,
            Err(_) => return None,
        };

        let cookie = v.get("raw_cookie");
        match cookie {
            Some(c) => Some(c.as_str().unwrap().to_string()),
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::login::Login;

    use super::*;

    #[tokio::test]
    async fn test_cookie_store() {
        let _ = Login::do_login().await;
        let cookie_store = CookieStore::load_raw().await;
        println!("{:?}", cookie_store);
    }
}
