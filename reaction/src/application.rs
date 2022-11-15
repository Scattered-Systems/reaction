/*
    Appellation: interface <module>
    Contributors: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
pub use super::states::State;
use crate::{api::Api, cli::CommandLineInterface, Context, Settings};
use scsys::prelude::BoxResult;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Application<T: Clone + Default + Display> {
    pub ctx: Context,
    pub settings: Settings,
    pub state: State<T>,
}

impl<T: Clone + Default + Display> Application<T> {
    pub fn new(settings: Settings) -> Self {
        let context = Context::new(settings.clone());
        Self {
            ctx: context,
            settings,
            state: Default::default(),
        }
    }
    pub fn setup_logger(&self) -> &Self {
        self.clone().settings.logger.unwrap_or_default().setup();

        self
    }
    pub fn set_state(&mut self, state: State<T>) -> &Self {
        self.state = state;
        self
    }
    pub async fn spawn_api(&self) -> BoxResult {
        let api = Api::new(self.ctx.clone());
        api.run().await?;
        Ok(())
    }
    pub fn cli(&self) -> CommandLineInterface {
        CommandLineInterface::default()
    }
    pub async fn run(&self) -> BoxResult<&Self> {
        self.setup_logger();
        tracing::info!("Success: Application initialized; awaiting commands...");
        let cli = CommandLineInterface::default();
        tracing::info!("Success: Commands parsed, processing requests...");
        cli.handler().await;
        Ok(self)
    }
}

impl<T: Clone + Default + Display + Serialize> std::fmt::Display for Application<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string_pretty(&self).unwrap())
    }
}
