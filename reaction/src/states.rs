/*
    Appellation: states <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use scsys::{
    prelude::{Message, Stateful},
    Timestamp,
};
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use strum::{EnumString, EnumVariantNames};
#[derive(
    Clone, Debug, Deserialize, EnumString, EnumVariantNames, Eq, Hash, PartialEq, Serialize,
)]
#[strum(serialize_all = "snake_case")]
pub enum States<T: Default + Display> {
    Request(T),
    Response(T),
    Idle,
}

impl<T: Default + Display> Default for States<T> {
    fn default() -> Self {
        Self::Idle
    }
}

impl<T: Default + Display + Serialize> Display for States<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct State<T: Default + Display> {
    pub message: Message<States<T>>,
    pub timestamp: String,
}

impl<T: Default + Display> State<T> {
    pub fn new(message: Message<States<T>>) -> Self {
        let timestamp = Timestamp::pretty();
        Self { message, timestamp }
    }
}

impl<T: Default + Display> Default for State<T> {
    fn default() -> Self {
        Self::new(Message::from(States::Idle))
    }
}

impl<T: Clone + Default + Serialize + std::fmt::Display> Stateful for State<T> {
    type Data = States<T>;

    fn message(&self) -> &Message<Self::Data> {
        &self.message
    }

    fn timestamp(&self) -> i64 {
        Timestamp::try_from(self.timestamp.clone().as_str())
            .ok()
            .unwrap()
            .into()
    }
}

impl<T: Default + Serialize + std::fmt::Display> std::fmt::Display for State<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_states() {
        let a = State::<String>::default();
        assert_eq!(a.message.data, Message::from(States::Idle).data)
    }

    #[test]
    fn test_request_state() {
        let a = State::<String>::new(Message::from(States::try_from("request").ok().unwrap()));
        let b = State::<String>::new(Message::from(States::Request(Default::default())));
        assert_eq!(a.message.data, b.message.data)
    }
}
