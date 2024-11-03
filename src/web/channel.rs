use bevy_channel_trigger::ChannelSender;
use std::sync::OnceLock;

use super::WebEvent;

static SENDER: OnceLock<Option<ChannelSender<WebEvent>>> = OnceLock::new();

//TODO: error logging
pub fn send_event(e: WebEvent) {
    SENDER
        .get()
        .expect("invalid sender lock")
        .as_ref()
        .expect("sender not found")
        .send(e);
}

pub fn set_sender(sender: ChannelSender<WebEvent>) {
    while SENDER.set(Some(sender.clone())).is_err() {}
}
