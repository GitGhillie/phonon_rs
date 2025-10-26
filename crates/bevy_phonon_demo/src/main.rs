use bevy::{
    anti_alias::fxaa::Fxaa,
    camera::Exposure,
    light::{
        AtmosphereEnvironmentMapLight, CascadeShadowConfigBuilder, DirectionalLightShadowMap,
        light_consts::lux,
    },
    pbr::{Atmosphere, AtmosphereSettings, ScreenSpaceAmbientOcclusion, ScreenSpaceReflections},
    post_process::bloom::Bloom,
    prelude::*,
    render::view::Hdr,
};

use bevy_editor_cam::prelude::*;
use bevy_skein::SkeinPlugin;

use crate::water::WaterPlugin;

mod water;

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
        ))
        .insert_resource(DirectionalLightShadowMap { size: 4096 })
        .insert_resource(ClearColor(Color::Srgba(Srgba {
            red: 0.02,
            green: 0.02,
            blue: 0.02,
            alpha: 1.0,
        })))
        .insert_resource(AmbientLight::NONE)
        .add_systems(Startup, setup)
        .add_systems(PostStartup, into_the_sky)
        .run();
}

/// Moves everything up so that the atmosphere looks it bit more atmospheric
fn into_the_sky(mut tfs: Query<&mut Transform>) {
    for mut tf in tfs.iter_mut() {
        tf.translation.y = tf.translation.y + 5000.0;
    }
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // cube
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
        Transform::from_xyz(0.0, 0.5, 0.0),
    ));
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
    // Add a fog volume.
    // commands.spawn((
    //     FogVolume::default(),
    //     Transform::from_scale(Vec3::splat(35.0)),
    // ));
}
