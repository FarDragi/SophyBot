use std::{collections::HashMap, sync::Arc, time::Duration};

use tokio::sync::RwLock;

use crate::config::Config;

#[derive(Debug, Default)]
pub struct State {
    pub config: Arc<Config>,
    pub shards: Arc<RwLock<HashMap<u64, Duration>>>,
}
