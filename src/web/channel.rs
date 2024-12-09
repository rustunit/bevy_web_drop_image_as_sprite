use bevy_channel_trigger::ChannelSender;
use std::sync::OnceLock;

use super::WebEvent;

static SENDER: OnceLock<Option<ChannelSender<WebEvent>>> = OnceLock::new();

pub fn send_event(e: WebEvent) {
    let Some(sender) = SENDER.get().map(Option::as_ref).flatten() else {
        return bevy::log::error!("`WebPlugin` not installed correctly (no sender found)");
    };
    sender.send(e);
}

pub fn set_sender(sender: ChannelSender<WebEvent>) {
    while SENDER.set(Some(sender.clone())).is_err() {}
}
