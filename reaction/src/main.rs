/*
    Appellation: aether <binary>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description:
        Aether is set to service the platform in a number of ways from being the primary method of interacting with Disarray's PoW half
*/
pub use self::{context::*, settings::*, states::*};

pub mod api;
pub mod cli;
pub(crate) mod context;
pub(crate) mod settings;
pub(crate) mod states;

use scsys::AsyncResult;
use std::{fmt::Display, sync::Arc};

#[tokio::main]
async fn main() -> AsyncResult {
    Application::<String>::default().start().await?;

    Ok(())
}

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct Application<T: Clone + Default + Display = String> {
    pub cnf: Settings,
    pub ctx: Context,
    pub state: Arc<State<T>>,
}

impl<T: Clone + Default + Display> Application<T> {
    pub fn new(cnf: Settings) -> Self {
        let ctx = Context::new(cnf.clone());
        let state = Arc::new(State::<T>::default());
        Self { cnf, ctx, state }
    }
    pub async fn setup(&mut self) -> AsyncResult<&Self> {
        // Initialize the logger
        self.clone().cnf.logger.setup(None);
        tracing_subscriber::fmt::init();

        tracing::info!("Success: Application initialized and awaiting commands");
        Ok(self)
    }
    pub fn set_state(&mut self, state: &State<T>) -> &Self {
        self.state = Arc::new(state.clone());
        self
    }
    /// Implements a graceful shutdown when users press CTRL + C
    pub async fn shutdown(&self) {
        tracing::info!("Signal Received: Terminating platform services...");
        tokio::signal::ctrl_c()
            .await
            .expect("Expect shutdown signal handler");
    }
    pub async fn start(&mut self) -> AsyncResult<&Self> {
        self.setup().await?;
        let cli = cli::new();
        tracing::info!("Success: Commands parsed, processing requests...");
        cli.handler().await?;
        Ok(self)
    }
}

impl<T: Clone + Default + Display + serde::Serialize> std::fmt::Display for Application<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            serde_json::json!({"cnf": self.cnf, "ctx": self.ctx})
        )
    }
}
