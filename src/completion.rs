//! completion module

use anyhow::Result;
use log::debug;

pub struct completion;

impl completion {
    pub fn new(_config: crate::config::ShellConfig) -> Result<Self> {
        debug!("Creating completion");
        Ok(Self)
    }
    
    pub async fn init(&mut self) -> Result<()> {
        debug!("Initializing completion");
        Ok(())
    }
    
    pub async fn shutdown(&mut self) -> Result<()> {
        debug!("Shutting down completion");
        Ok(())
    }
}
