// TODO
use lightyear::prelude::*;

mod components;
mod messages;

pub use components::*;
pub use messages::*;

protocolize! {
    Self = OrvinProtocol,
    Message = messages::Messages,
    Component = Components,
}

pub fn protocol() -> OrvinProtocol {
    let mut protocol = OrvinProtocol::default();
    protocol.add_channel::<MainChannel>(ChannelSettings {
        mode: ChannelMode::OrderedReliable(ReliableSettings::default()),
        direction: ChannelDirection::Bidirectional,
    });
    protocol
}

#[derive(Channel)]
pub struct MainChannel;
