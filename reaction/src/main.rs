/*
   Appellation: Flow <binary>
   Creator: FL03 <jo3mccain@icloud.com>
   Description:
       ... Summary ...

*/
extern crate dotenv;
pub use self::{application::*, context::*, settings::*, states::*};

pub mod api;
pub(crate) mod application;
pub mod cli;
pub(crate) mod context;
pub(crate) mod settings;
pub(crate) mod states;

#[tokio::main]
async fn main() -> scsys::prelude::BoxResult {
    dotenv::dotenv()?;
    Application::<String>::default().run().await?;

    Ok(())
}
