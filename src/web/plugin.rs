use bevy::prelude::*;

#[cfg_attr(not(target_family = "wasm"), allow(dead_code))]
#[derive(Event, Clone, Debug)]
pub enum WebEvent {
    Drop {
        name: String,
        data: Vec<u8>,
        mime_type: String,
    },
}

pub struct WebPlugin {
    #[cfg_attr(not(target_family = "wasm"), allow(dead_code))]
    pub dom_drop_element_id: String,
}
impl Plugin for WebPlugin {
    #[cfg_attr(not(target_family = "wasm"), allow(unused_variables))]
    fn build(&self, app: &mut App) {
        #[cfg(target_family = "wasm")]
        {
            use bevy_channel_trigger::ChannelTriggerApp;

            let sender = app.add_channel_trigger::<WebEvent>();

            super::channel::set_sender(sender);

            super::web::register_drop(&self.dom_drop_element_id).unwrap();
        }
    }
}
