//! SigmaOS Bash Shell Shell Compatibility Layer
//!
//! Bash Shell shell compatibility for SigmaOS
//! Provides command-line shell and scripting capabilities

#![forbid(unsafe_code)]

use anyhow::Result;
use log::{debug, error, info, warn};
use std::sync::Arc;
use tokio::sync::RwLock;

pub mod parser;
pub mod executor;
pub mod builtin;
pub mod completion;
pub mod history;
pub mod alias;
pub mod env;
pub mod prompt;
pub mod config;
pub mod error;

pub use parser::*;
pub use executor::*;
pub use builtin::*;
pub use completion::*;
pub use history::*;
pub use alias::*;
pub use env::*;
pub use prompt::*;
pub use config::*;
pub use error::*;

/// Shell compatibility layer
pub struct Shell {
    pub config: ShellConfig,
    pub parser: Arc<RwLock<Parser>>,
    pub executor: Arc<RwLock<Executor>>,
    pub history: Arc<RwLock<History>>,
}

impl Shell {
    /// Create new shell instance
    pub fn new() -> Result<Self> {
        info!("Initializing SigmaOS shell");
        let config = ShellConfig::load_default()?;
        debug!("Loaded shell configuration");
        
        let parser = Arc::new(RwLock::new(Parser::new(config.clone())?));
        let executor = Arc::new(RwLock::new(Executor::new(config.clone())?));
        let history = Arc::new(RwLock::new(History::new(config.clone())?));
        
        Ok(Self {
            config,
            parser,
            executor,
            history,
        })
    }

    /// Initialize shell
    pub async fn init(&mut self) -> Result<()> {
        info!("Starting shell initialization");
        
        let mut parser = self.parser.write().await;
        parser.init().await?;
        debug!("Parser initialized");
        
        let mut executor = self.executor.write().await;
        executor.init().await?;
        debug!("Executor initialized");
        
        let mut history = self.history.write().await;
        history.load().await?;
        debug!("History loaded");
        
        info!("Shell initialization complete");
        Ok(())
    }

    /// Execute command
    pub async fn execute(&mut self, input: &str) -> Result<i32> {
        debug!("Executing command: {}", input);
        
        let mut parser = self.parser.write().await;
        let commands = parser.parse(input)?;
        drop(parser);
        
        let mut executor = self.executor.write().await;
        let exit_code = executor.execute(commands).await?;
        
        let mut history = self.history.write().await;
        history.add(input, exit_code).await?;
        
        debug!("Command executed with exit code: {}", exit_code);
        Ok(exit_code)
    }

    /// Run interactive shell
    pub async fn run_interactive(&mut self) -> Result<()> {
        info!("Starting interactive shell");
        
        loop {
            let prompt = self.config.get_prompt();
            print!("{}", prompt);
            
            let mut input = String::new();
            std::io::stdin().read_line(&mut input)?;
            let input = input.trim();
            
            if input.is_empty() {
                continue;
            }
            
            self.execute(input).await?;
        }
    }

    /// Shutdown shell
    pub async fn shutdown(&mut self) -> Result<()> {
        info!("Shutting down shell");
        
        let mut history = self.history.write().await;
        history.save().await?;
        
        info!("Shell shutdown complete");
        Ok(())
    }
}
