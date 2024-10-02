# Steam Audio Rust Rewrite

A community effort to rewrite Valve's [Steam Audio] into a Rust library.

Note: Steam Audio is massive and the benefits of rewriting it in Rust are going to be sparse at best.
Therefore, this is more of a project for people passionate about Rust + audio/maths/physics.
If you want a quicker route to using Steam Audio in your Rust project, it is better to use one of the Rust bindings out there.

## Status

The following user-facing effects have been ported:
- Panning Effect (stereo only for now)
- Direct Effect
  - Distance attenuation
  - Air absorption
  - Occlusion
  - Transmission (one material per mesh for now)
  - Directivity

Game engine developers can use the following to integrate the effects:
- [FMOD integration]
- [Kira integration]

Game developers can use the [Bevy integration].

Feel free to open a PR to add an integration!

## Contributing

Please contact me on Discord (user: ixml) to check what is being worked on and what needs to be done.

At this stage the focus is on the following:
- Try to get the tests and benchmarks working
- Write Rusty code
- Don't worry too much about performance yet (leave a 'todo' if necessary)
- Look for opportunities to make use of the Rust ecosystem

## License

Licensed under Apache-2.0

[Steam Audio]: https://github.com/ValveSoftware/steam-audio
[FMOD integration]: https://crates.io/crates/phonon-fmod
[Kira integration]: https://crates.io/crates/phonon-kira
[Bevy integration]: https://github.com/GitGhillie/bevy_phonon
