use super::OrvinProtocol;
use bevy::prelude::*;
use lightyear::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Component, Message, Deserialize, Serialize, Clone, PartialEq)]
pub struct PlayerId(ClientId);

#[component_protocol(protocol = "OrvinProtocol")]
pub enum Components {
    PlayerId(PlayerId),
}
