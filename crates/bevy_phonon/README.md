# Bevy Phonon integration

Bevy Phonon integration using Firewheel and bevy_seedling.
Please see https://github.com/GitGhillie/phonon_rs for more information about Phonon.

## Usage

Make sure you are setup with bevy_seedling first. Then add the Phonon plugin:

```rust
    .add_plugins(SeedlingPlugin::default())
    .add_plugins(PhononPlugin::default())
    .add_plugins(bevy_phonon::debug::DebugPlugin) // Optional
```

The DebugPlugin adds an AudioGizmoConfigGroup, which can be used like any other [GizmoConfigGroup].

Add the AudioListener component to your camera:

```rust
    commands.spawn((
        Camera3d::default(),
        AudioListener,
    ));
```

Add a bevy_seedling sample player with phonon spatializer effect:

```rust
    commands.spawn((
        SamplePlayer::new(demo_assets.audio_sample.clone()).looping(),
        sample_effects![SpatializerNode::default()],
        Transform::from_xyz(0.0, 0.5, 0.0),
    ));
```

The spatializer nodes includes a selection of Steam Audio effects with opinionated defaults.
Checkout the Bevy Phonon demo if you want to see how to configure the effects.

Todo: Show how to add materials (check the demo for now).

[GizmoConfigGroup]: https://docs.rs/bevy/latest/bevy/gizmos/config/trait.GizmoConfigGroup.html.
