use bevy::{
    camera_controller::free_camera::{FreeCamera, FreeCameraPlugin},
    diagnostic::LogDiagnosticsPlugin,
    input::common_conditions::input_just_pressed,
    light::{CascadeShadowConfigBuilder, SunDisk, light_consts::lux},
    pbr::ScatteringMedium,
    prelude::*,
    window::{CursorGrabMode, CursorOptions},
};

use avian3d::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};
use bevy_phonon::{AudioListener, effects::spatializer::SpatializerNode, prelude::PhononPlugin};
use bevy_seedling::{SeedlingPlugin, node::RegisterNode, sample::AudioSample};
use bevy_skein::SkeinPlugin;

use crate::water::WaterPlugin;

mod graphics;
mod scene_switching;
mod scenes;
mod water;

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
enum AssetLoadingState {
    #[default]
    Loading,
    Loaded,
}

#[derive(AssetCollection, Resource)]
struct DemoAssets {
    #[asset(path = "audio/dpren_very-lush-and-swag-loop.ogg")]
    audio_sample: Handle<AudioSample>,
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Bevy Phonon demo".into(),
                    ..default()
                }),
                ..default()
            }),
            graphics::GraphicsPlugin,
            scene_switching::ScenePlugin,
            PhysicsPlugins::default(),
            FreeCameraPlugin,
            WaterPlugin,
            SkeinPlugin::default(),
            EguiPlugin::default(),
            WorldInspectorPlugin::new(),
        ))
        .add_plugins(SeedlingPlugin::default())
        .add_plugins(PhononPlugin::default())
        .add_plugins(bevy_phonon::debug::DebugPlugin) // Optional
        .add_plugins(LogDiagnosticsPlugin::default())
        .register_node::<SpatializerNode>()
        .init_state::<AssetLoadingState>()
        .add_loading_state(
            LoadingState::new(AssetLoadingState::Loading)
                .continue_to_state(AssetLoadingState::Loaded)
                .load_collection::<DemoAssets>(),
        )
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            release_cursor.run_if(input_just_pressed(KeyCode::Escape)),
        )
        .run();
}

/// Setup the common parts between the different scenes in this demo
fn setup(mut commands: Commands, scattering_mediums: ResMut<Assets<ScatteringMedium>>) {
    // Spawn the player camera
    commands.spawn((
        Name::from("Camera"),
        Camera3d::default(),
        graphics::camera_components(scattering_mediums),
        AudioListener,
        FreeCamera::default(),
        Transform::from_xyz(-2.5, 1.8, 0.5).looking_at(Vec3::new(0.0, 1.0, 0.0), Vec3::Y),
    ));

    // Spawn the floor collider
    commands.spawn((
        Name::from("Floor"),
        RigidBody::Static,
        Collider::cuboid(100.0, 0.1, 100.0),
    ));

    // Sun
    commands.spawn((
        Name::from("Sun"),
        DirectionalLight {
            shadows_enabled: true,
            illuminance: lux::RAW_SUNLIGHT,
            ..default()
        },
        Transform::from_xyz(1.0, 0.2, 0.3).looking_at(Vec3::ZERO, Vec3::Y),
        CascadeShadowConfigBuilder::default().build(),
        SunDisk::EARTH,
        //VolumetricLight,
    ));
}

fn release_cursor(mut cursor: Single<&mut CursorOptions>) {
    cursor.visible = true;
    cursor.grab_mode = CursorGrabMode::None;
}
