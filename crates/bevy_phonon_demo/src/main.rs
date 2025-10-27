use bevy::{
    anti_alias::fxaa::Fxaa,
    camera::Exposure,
    light::{AtmosphereEnvironmentMapLight, DirectionalLightShadowMap},
    pbr::{Atmosphere, AtmosphereSettings, ScreenSpaceAmbientOcclusion, ScreenSpaceReflections},
    post_process::bloom::Bloom,
    prelude::*,
    render::view::Hdr,
};

use bevy_asset_loader::prelude::*;
use bevy_editor_cam::prelude::*;
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};
use bevy_skein::SkeinPlugin;

use crate::water::WaterPlugin;

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
    #[asset(path = "audio/background.ogg")]
    background: Handle<Gltf>,
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
            MeshPickingPlugin,
            DefaultEditorCamPlugins,
            WaterPlugin,
            SkeinPlugin::default(),
            EguiPlugin::default(),
            WorldInspectorPlugin::new(),
        ))
        .insert_resource(DirectionalLightShadowMap { size: 4096 })
        .insert_resource(ClearColor(Color::Srgba(Srgba {
            red: 0.02,
            green: 0.02,
            blue: 0.02,
            alpha: 1.0,
        })))
        .insert_resource(AmbientLight::NONE)
        .add_loading_state(
            LoadingState::new(AssetLoadingState::Loading)
                .continue_to_state(AssetLoadingState::Loaded)
                .load_collection::<DemoAssets>(),
        )
        .add_systems(OnEnter(AssetLoadingState::Loaded), setup_scene1)
        .add_systems(Startup, setup)
        .add_systems(Startup, scenes::intro::setup)
        .add_systems(PostStartup, into_the_sky)
        .run();
}

// todo move this, adjust the scale
/// Moves everything up so that the atmosphere looks it bit more atmospheric
fn into_the_sky(mut tfs: Query<&mut Transform>) {
    for mut tf in tfs.iter_mut() {
        tf.translation.y = tf.translation.y + 5000.0;
    }
}

/// Setup the common parts between the different scenes in this demo
fn setup(mut commands: Commands) {
    // camera
    commands.spawn((
        EditorCam::default(),
        Camera3d::default(),
        Hdr,
        Msaa::Off,
        ScreenSpaceAmbientOcclusion::default(),
        ScreenSpaceReflections::default(),
        Fxaa::default(),
        // This is the component that enables atmospheric scattering for a camera
        Atmosphere::EARTH,
        // The scene is in units of 10km, so we need to scale up the
        // aerial view lut distance and set the scene scale accordingly.
        // Most usages of this feature will not need to adjust this.
        AtmosphereSettings::default(),
        // The directional light illuminance used in this scene
        // (the one recommended for use with this feature) is
        // quite bright, so raising the exposure compensation helps
        // bring the scene to a nicer brightness range.
        Exposure::SUNLIGHT,
        // Tonemapper chosen just because it looked good with the scene, any
        // tonemapper would be fine :)
        //Tonemapping::AcesFitted,
        // Bloom gives the sun a much more natural look.
        Bloom::NATURAL,
        // Enables the atmosphere to drive reflections and ambient lighting (IBL) for this view
        AtmosphereEnvironmentMapLight::default(),
        //VolumetricFog::default(),
        Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}
