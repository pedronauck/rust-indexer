use crate::prelude::*;
use once_cell::sync::Lazy;
use std::sync::Mutex;

static INSTANCE: Lazy<Mutex<Option<Env>>> = Lazy::new(|| Mutex::new(None));

#[derive(Debug, Clone)]
pub struct Env {
    pub node_url: String,
}

impl Env {
    pub fn init() -> DynResult<()> {
        dotenvy::dotenv()?;

        let mut instance = INSTANCE.lock().unwrap();
        if instance.is_some() {
            panic!("Env instance already initialized");
        }
        *instance = Some(Self {
            node_url: Self::get_from_env("NODE_URL"),
        });
        Ok(())
    }

    pub fn get() -> Env {
        INSTANCE
            .lock()
            .unwrap()
            .as_ref()
            .expect("Env instance not initialized")
            .clone()
    }

    fn get_from_env(key: &str) -> String {
        dotenvy::var(key).unwrap_or_else(|_| panic!("{} must be set", key))
    }
}
