mod web;

use bevy::{
    image::{CompressedImageFormats, ImageSampler, ImageType},
    prelude::*,
    render::render_asset::RenderAssetUsages,
    window::WindowResolution,
};
use web::{WebEvent, WebPlugin};

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(AssetPlugin {
                    meta_check: bevy::asset::AssetMetaCheck::Never,
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "demo".to_string(),
                        // Bind to canvas included in `index.html`
                        canvas: Some("#bevy".to_owned()),
                        // Tells wasm not to override default event handling, like F5 and Ctrl+R
                        prevent_default_event_handling: false,
                        resolution: WindowResolution::new(500.0, 500.0),
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_plugins(WebPlugin {
            dom_drop_element_id: String::from("bevy"),
        })
        .add_systems(Startup, setup)
        .add_systems(Update, sprite_movement)
        .add_observer(process_web_events)
        .run();
}

#[derive(Component)]
enum Direction {
    Up,
    Down,
}

// start by loading an image from the assets folder we ship with
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((Camera2d, Msaa::Off));
    commands.spawn((
        Transform::from_xyz(-10., 0., 0.),
        Sprite::from_image(asset_server.load("icon.png")),
        Direction::Up,
    ));
}

/// make the sprite move to show interactivity or if something is blocking the main thread
fn sprite_movement(time: Res<Time>, mut sprite_position: Query<(&mut Direction, &mut Transform)>) {
    for (mut logo, mut transform) in &mut sprite_position {
        match *logo {
            Direction::Up => transform.translation.y += 150. * time.delta_secs(),
            Direction::Down => transform.translation.y -= 150. * time.delta_secs(),
        }

        if transform.translation.y > 100. {
            *logo = Direction::Down;
        } else if transform.translation.y < -100. {
            *logo = Direction::Up;
        }
    }
}

fn process_web_events(
    trigger: Trigger<WebEvent>,
    assets: Res<AssetServer>,
    mut sprite: Single<&mut Sprite>,
) {
    let e = trigger.event();
    let WebEvent::Drop {
        data,
        mime_type,
        name,
    } = e;

    let Ok(image) = Image::from_buffer(
        data,
        ImageType::MimeType(mime_type),
        CompressedImageFormats::default(),
        true,
        ImageSampler::Default,
        RenderAssetUsages::RENDER_WORLD,
    ) else {
        info!("could not load image: '{name}' of type {mime_type}");
        return;
    };

    let handle = assets.add(image);

    info!("loaded image: '{name}'");

    sprite.image = handle;
}
