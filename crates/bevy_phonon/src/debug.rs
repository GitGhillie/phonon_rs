use bevy::color::{
    Color,
    palettes::basic::{GREEN, RED, SILVER, YELLOW},
};
use bevy::prelude::*;
use bevy_seedling::{
    prelude::{EffectsQuery, SampleEffects},
    sample::SamplePlayer,
};
use firewheel_phonon::effects::spatializer::SpatializerNode;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.init_gizmo_group::<AudioGizmoConfigGroup>().add_systems(
            PostUpdate,
            visualize_sources.after(TransformSystems::Propagate),
        );
    }
}

/// The [`GizmoConfigGroup`] used to configure the visualization of audio entities/geometry.
#[derive(Clone, Reflect, GizmoConfigGroup)]
#[reflect(Clone, Default)]
pub struct AudioGizmoConfigGroup {
    /// Draw a gizmo for all audio objects if true.
    ///
    /// Defaults to `false`.
    pub draw_all: bool,
    /// [`Color`] to use for drawing a [`AudioListener`] gizmo.
    ///
    /// Defaults to [`GREEN`].
    pub audio_listener_color: Color,
    /// [`Color`] to use for drawing a [`AudioSource`] gizmo.
    ///
    /// Defaults to [`RED`].
    pub audio_source_color: Color,
    /// [`Color`] to use for drawing a [`DirectionalLight`] gizmo when [`LightGizmoColor::ByLightType`] is used.
    ///
    /// Defaults to [`YELLOW`].
    pub geometry_color: Color,
}

impl Default for AudioGizmoConfigGroup {
    fn default() -> Self {
        Self {
            draw_all: false,
            audio_listener_color: GREEN.into(),
            audio_source_color: RED.into(),
            geometry_color: YELLOW.into(),
        }
    }
}

fn visualize_sources(
    mut gizmos: Gizmos<AudioGizmoConfigGroup>,
    audio_sources: Query<(&SampleEffects, &GlobalTransform), With<SamplePlayer>>,
    effects: Query<&SpatializerNode>,
) -> Result {
    for (player, transform) in audio_sources {
        let effect = effects.get_effect(&player)?;
        let radius = effect.simulator_settings.occlusion_radius;
        let translation = transform.translation();

        gizmos.sphere(Isometry3d::from_translation(translation), radius, RED);

        if effect.direct_effect_parameters.flags.directivity {
            let arrow_start = translation;
            let arrow_end = translation + (*transform.forward() * radius);
            gizmos.arrow(arrow_start, arrow_end, SILVER);
        }
    }

    Ok(())
}
