use bevy_channel_trigger::ChannelSender;
use std::sync::OnceLock;

use super::WebEvent;

static SENDER: OnceLock<ChannelSender<WebEvent>> = OnceLock::new();

pub fn send_event(e: WebEvent) {
    let Some(sender) = SENDER.get() else {
        return bevy::log::error!("`WebPlugin` not installed correctly (no sender found)");
    };
    sender.send(e);
}

pub fn set_sender(sender: ChannelSender<WebEvent>) {
    if SENDER.set(sender).is_err() {
        bevy::log::error!("`WebPlugin` installed more than once");
    }
}
