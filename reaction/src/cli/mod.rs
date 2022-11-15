/*
    Appellation: cli <module>
    Contributors: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
pub use self::{args::Power, commands::Commands, interface::CommandLineInterface};

mod args;
mod commands;
mod interface;
