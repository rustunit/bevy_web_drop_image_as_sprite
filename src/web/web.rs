use bevy::prelude::*;
use gloo::events::{EventListener, EventListenerOptions};
use web_sys::{wasm_bindgen::JsCast, DragEvent, FileReader};

use crate::web::channel::send_event;

pub fn register_drop(id: &str) -> Option<()> {
    let doc = gloo::utils::document();
    let element = doc.get_element_by_id(id)?;

    EventListener::new_with_options(
        &element,
        "dragover",
        EventListenerOptions::enable_prevent_default(),
        move |event| {
            let event: DragEvent = event.clone().dyn_into().expect("wrong event type");
            event.stop_propagation();
            event.prevent_default();

            event
                .data_transfer()
                .expect("invalid data transfer")
                .set_drop_effect("copy");
            event
                .data_transfer()
                .expect("invalid data transfer")
                .set_effect_allowed("all");

            info!("dragover event",);
        },
    )
    .forget();

    EventListener::new_with_options(
        &element,
        "drop",
        EventListenerOptions::enable_prevent_default(),
        move |event| {
            let event: DragEvent = event.clone().dyn_into().expect("rwong event type");
            event.stop_propagation();
            event.prevent_default();

            info!("drop event");

            let transfer = event.data_transfer().expect("invalid data transfer");
            let files = transfer.items();

            for idx in 0..files.length() {
                let file = files.get(idx).expect("invalid item");
                let file_info = file.get_as_file().expect("not a file").unwrap();

                info!(
                    "file[{idx}] = '{}' - {} - {} b",
                    file_info.name(),
                    file_info.type_(),
                    file_info.size()
                );

                let file_reader = FileReader::new().unwrap();

                {
                    let file_reader = file_reader.clone();
                    let file_info = file_info.clone();
                    EventListener::new(&file_reader.clone(), "load", move |_event| {
                        let result = file_reader.result().expect("result invalid");
                        let result = web_sys::js_sys::Uint8Array::new(&result);
                        let mut data: Vec<u8> = vec![0; result.length() as usize];
                        result.copy_to(&mut data);

                        info!("drop file read: {}", file_info.name());

                        send_event(crate::web::WebEvent::Drop(file_info.name(), data));
                    })
                    .forget();
                }

                file_reader.read_as_array_buffer(&file_info).unwrap();
            }

            info!("dragover event");
        },
    )
    .forget();

    info!("drag handlers installed");

    Some(())
}
