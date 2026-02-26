use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;

#[derive(Clone)]
pub struct Store {
    inner: Arc<RwLock<HashMap<Vec<u8>, Vec<u8>>>>,
}

impl Store {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn set(&self, key: Vec<u8>, value: Vec<u8>) {
        let mut map = self.inner.write().await;
        map.insert(key, value);
    }

    pub async fn get(&self, key: &[u8]) -> Option<Vec<u8>> {
        let map = self.inner.read().await;
        map.get(key).cloned()
    }

    pub async fn del(&self, key: &[u8]) -> bool {
        let mut map = self.inner.write().await;
        map.remove(key).is_some()
    }
}
