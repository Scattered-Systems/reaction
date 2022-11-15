/*
    Appellation: states <module>
    Contributors: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
use scsys::prelude::{messages::Message, Stateful, Timestamp};
use serde::{Deserialize, Serialize};
use strum::{EnumString, EnumVariantNames};

#[derive(
    Clone, Debug, Deserialize, EnumString, EnumVariantNames, Eq, Hash, PartialEq, Serialize,
)]
#[strum(serialize_all = "snake_case")]
pub enum States<T: Default + std::fmt::Display> {
    Request(State<T>),
    Response(State<T>),
    Idle,
}

impl<T: Default + std::fmt::Display> Default for States<T> {
    fn default() -> Self {
        Self::Idle
    }
}

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct State<T: Default + std::fmt::Display> {
    pub message: Message<T>,
    pub timestamp: Timestamp,
}

impl<T: Default + std::fmt::Display> State<T> {
    pub fn new(message: Message<T>) -> Self {
        Self {
            message,
            timestamp: Default::default(),
        }
    }
}

impl<T: Clone + Default + Serialize + std::fmt::Display> Stateful for State<T> {
    type Data = T;

    fn message(&self) -> &Message<Self::Data> {
        &self.message
    }

    fn timestamp(&self) -> i64 {
        self.timestamp.clone().into()
    }
}

impl<T: Default + Serialize + std::fmt::Display> std::fmt::Display for State<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string_pretty(&self).unwrap())
    }
}
