# FMOD Plugin for phonon_rs

Please see https://github.com/GitGhillie/phonon_rs for a list of features and game engine integrations.
Use this crate to integrate phonon_rs with your game engine if you are using FMOD.

While it is not necessary for this plugin to link with the FMOD libraries,
`libfmod` does require it at the moment (see [issue](https://github.com/lebedec/libfmod/issues/15)).
See https://github.com/lebedec/libfmod for instructions on installing the FMOD libraries.

Once built the dylib can be loaded into FMOD Studio and the application:

For FMOD Studio place it in one of the folders indicated here:
https://www.fmod.com/docs/2.02/studio/plugin-reference.html#loading-plug-ins

Place phonon_fmod.plugin.js in the same folder to give the plugin a user-friendly appearance inside FMOD.

On the application side the plugin can either be dynamically or statically linked.

![FMOD Phonon Spatializer](/media/phonon-spatializer.png)
