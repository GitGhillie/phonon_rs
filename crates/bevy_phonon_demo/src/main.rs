use bevy::{
    light::{CascadeShadowConfigBuilder, light_consts::lux},
    prelude::*,
};

use bevy_asset_loader::prelude::*;
use bevy_editor_cam::prelude::*;
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
            MeshPickingPlugin,
            DefaultEditorCamPlugins,
            WaterPlugin,
            SkeinPlugin::default(),
            EguiPlugin::default(),
            WorldInspectorPlugin::new(),
        ))
        .add_plugins(SeedlingPlugin::default())
        .add_plugins(PhononPlugin::default())
        .register_node::<SpatializerNode>()
        .init_state::<AssetLoadingState>()
        .add_loading_state(
            LoadingState::new(AssetLoadingState::Loading)
                .continue_to_state(AssetLoadingState::Loaded)
                .load_collection::<DemoAssets>(),
        )
        .add_systems(Startup, setup)
        .run();
}

/// Setup the common parts between the different scenes in this demo
fn setup(mut commands: Commands) {
    // camera
    commands.spawn((
        EditorCam::default(),
        Camera3d::default(),
        graphics::camera_components(),
        AudioListener,
        Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // Sun
    commands.spawn((
        DirectionalLight {
            shadows_enabled: true,
            illuminance: lux::RAW_SUNLIGHT,
            ..default()
        },
        Transform::from_xyz(1.0, 0.2, 0.3).looking_at(Vec3::ZERO, Vec3::Y),
        CascadeShadowConfigBuilder::default().build(),
        //VolumetricLight,
    ));
}
