use bevy::{
    anti_alias::fxaa::Fxaa,
    camera::Exposure,
    light::{AtmosphereEnvironmentMapLight, DirectionalLightShadowMap},
    pbr::{
        Atmosphere, AtmosphereSettings, ScatteringMedium, ScreenSpaceAmbientOcclusion,
        ScreenSpaceReflections,
    },
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
            .insert_resource(GlobalAmbientLight::NONE);
    }
}

pub(crate) fn camera_components(
    mut scattering_mediums: ResMut<Assets<ScatteringMedium>>,
) -> impl Bundle {
    (
        Hdr,
        Msaa::Off,
        ScreenSpaceAmbientOcclusion::default(),
        ScreenSpaceReflections::default(),
        Fxaa::default(),
        Atmosphere::earthlike(scattering_mediums.add(ScatteringMedium::default())),
        AtmosphereSettings::default(),
        Exposure::SUNLIGHT,
        //Tonemapping::AcesFitted,
        Bloom::NATURAL,
        AtmosphereEnvironmentMapLight::default(),
        //VolumetricFog::default(),
    )
}
