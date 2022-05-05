use std::sync::Arc;

use crate::config::Config;

#[derive(Debug)]
pub struct State {
    pub config: Arc<Config>,
}
