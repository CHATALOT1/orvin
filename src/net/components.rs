use super::OrvinProtocol;
use bevy::prelude::*;
use lightyear::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Component, Deserialize, Serialize)]
pub struct PlayerId(ClientId);

#[component_protocol(protocol = "OrvinProtocol")]
pub enum Components {}
