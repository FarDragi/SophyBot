use std::{collections::HashMap, sync::Arc, time::Duration};

use tokio::sync::RwLock;

use crate::{cache::Cache, config::Config, service::Service};

#[derive(Debug)]
pub struct State {
    pub config: Arc<Config>,
    pub shards: Arc<RwLock<HashMap<u64, Duration>>>,
    pub service: Arc<Service>,
    pub cache: Cache,
}
