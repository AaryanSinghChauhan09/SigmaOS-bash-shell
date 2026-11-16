//! compositor module

use anyhow::Result;
use log::debug;

pub struct compositor;

impl compositor {
    pub fn new(_config: crate::config::DisplayServerConfig) -> Result<Self> {
        debug!("Creating compositor");
        Ok(Self)
    }
    
    pub async fn init(&mut self) -> Result<()> {
        debug!("Initializing compositor");
        Ok(())
    }
    
    pub async fn shutdown(&mut self) -> Result<()> {
        debug!("Shutting down compositor");
        Ok(())
    }
}
