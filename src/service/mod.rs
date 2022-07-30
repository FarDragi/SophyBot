use std::sync::Arc;

use tokio::sync::Mutex;
use tonic::transport::Channel;

use crate::{
    config::Config,
    error::{AppError, MapError},
};

use self::proto::xp_client::XpClient;

pub mod xp;

mod proto {
    tonic::include_proto!("core");
}

#[derive(Debug)]
pub struct Service {
    xp: Mutex<XpClient<Channel>>,
}

impl Service {
    pub async fn new(config: Arc<Config>) -> Result<Self, AppError> {
        let url = config
            .service
            .url
            .clone()
            .unwrap_or_else(|| "http://localhost:50051".to_string());

        let xp_client = XpClient::connect(url.clone()).await.map_app_err()?;

        info!("Connected to service: {}", url);

        Ok(Self {
            xp: Mutex::new(xp_client),
        })
    }
}
