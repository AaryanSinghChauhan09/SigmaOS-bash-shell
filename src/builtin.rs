//! builtin module

use anyhow::Result;
use log::debug;

pub struct builtin;

impl builtin {
    pub fn new(_config: crate::config::ShellConfig) -> Result<Self> {
        debug!("Creating builtin");
        Ok(Self)
    }
    
    pub async fn init(&mut self) -> Result<()> {
        debug!("Initializing builtin");
        Ok(())
    }
    
    pub async fn shutdown(&mut self) -> Result<()> {
        debug!("Shutting down builtin");
        Ok(())
    }
}
