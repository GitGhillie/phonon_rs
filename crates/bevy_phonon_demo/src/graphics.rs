use bevy::{
    anti_alias::fxaa::Fxaa,
    camera::Exposure,
    light::{AtmosphereEnvironmentMapLight, DirectionalLightShadowMap},
    pbr::{Atmosphere, AtmosphereSettings, ScreenSpaceAmbientOcclusion, ScreenSpaceReflections},
    post_process::bloom::Bloom,
    prelude::*,
    render::view::Hdr,
};

pub(crate) struct GraphicsPlugin;

impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(DirectionalLightShadowMap { size: 4096 })
            .insert_resource(ClearColor(Color::Srgba(Srgba {
                red: 0.02,
                green: 0.02,
                blue: 0.02,
                alpha: 1.0,
            })))
            .insert_resource(AmbientLight::NONE);
    }
}

pub(crate) fn camera_components() -> impl Bundle {
    (
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
    )
}
