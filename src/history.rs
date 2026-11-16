//! history module

use anyhow::Result;
use log::debug;

pub struct history;

impl history {
    pub fn new(_config: crate::config::ShellConfig) -> Result<Self> {
        debug!("Creating history");
        Ok(Self)
    }
    
    pub async fn init(&mut self) -> Result<()> {
        debug!("Initializing history");
        Ok(())
    }
    
    pub async fn shutdown(&mut self) -> Result<()> {
        debug!("Shutting down history");
        Ok(())
    }
}
