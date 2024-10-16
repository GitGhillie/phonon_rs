//
// Copyright 2017-2023 Valve Corporation.
// Copyright 2024 phonon_rs contributors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//

use strum::FromRepr;

// The Params enum shown here is a work in progress. Still deciding if it's necessary to match
// the original from Steam Audio. The original is at the end of this file.

/// All parameters of the Phonon Spatializer which are available to users and FMOD internals.
// Any changes here must be reflected in the DSP description parameter list!
#[derive(FromRepr, Debug, PartialEq)]
#[repr(i32)]
pub enum Params {
    /**
     *  **Type**: `FMOD_DSP_PARAMETER_TYPE_DATA`
     *
     *  World-space position of the source. Automatically written by FMOD Studio.
     */
    SourcePosition = 0,

    /**
     *  **Type**: `FMOD_DSP_PARAMETER_TYPE_DATA`
     *
     *  Overall linear gain of this effect. Automatically read by FMOD Studio.
     */
    OverallGain = 1,

    /**
     *  **Type**: `FMOD_DSP_PARAMETER_TYPE_INT`
     *
     *  **Range**: 0 to 2.
     *
     *  How to render distance attenuation.
     *
     *  -   `0`: Don't render distance attenuation.
     *  -   `1`: Use a distance attenuation value calculated using the default physics-based model.
     *  -   `2`: Use a distance attenuation value calculated using the curve specified in the FMOD Studio UI.
     */
    ApplyDistanceAttenuation = 2,

    /**
     *  **Type**: `FMOD_DSP_PARAMETER_TYPE_INT`
     *
     *  **Range**: 0 to 2.
     *
     *  How to render air absorption.
     *
     *  -   `0`: Don't render air absorption.
     *  -   `1`: Use air absorption values calculated using the default exponential decay model.
     *  -   `2`: Use air absorption values specified in the \c AirabsorptionLow, \c AIRABSORPTION_MID, and
     *           \c AIRABSORPTION_HIGH parameters.
     */
    ApplyAirabsorption,

    /**
     *  **Type**: `FMOD_DSP_PARAMETER_TYPE_INT`
     *
     *  **Range**: 0 to 2.
     *
     *  How to render directivity.
     *
     *  -   `0`: Don't render directivity.
     *  -   `1`: Use a directivity value calculated using the default dipole model, driven by the
     *           \c DirectivityDipoleweight and \c DIRECTIVITY_DIPOLEPOWER parameters.
     *  -   `2`: Use the directivity value specified in the \c DIRECTIVITY parameter.
     */
    ApplyDirectivity,

    /**
     *  **Type**: `FMOD_DSP_PARAMETER_TYPE_INT`
     *
     *  **Range**: 0 to 2.
     *
     *  How to render occlusion.
     *
     *  -   `0`: Don't render occlusion.
     *  -   `1`: Use the occlusion value calculated by the game engine using simulation, and provided via the
     *           \c SimulationOutputs parameter.
     *  -   `2`: Use the occlusion value specified in the \c OCCLUSION parameter.
     */
    ApplyOcclusion,

    /**
     *  **Type**: `FMOD_DSP_PARAMETER_TYPE_INT`
     *
     *  **Range**: 0 to 2.
     *
     *  How to render transmission.
     *
     *  -   `0`: Don't render transmission.
     *  -   `1`: Use the transmission values calculated by the game engine using simulation, and provided via the
     *           \c SimulationOutputs parameter.
     *  -   `2`: Use the transmission values specified in the \c TransmissionLow, \c TRANSMISSION_MID, and
     *           \c TransmissionHigh parameters.
     */
    ApplyTransmission,

    /**
     *  **Type**: `FMOD_DSP_PARAMETER_TYPE_BOOL`
     *
     *  If true, reflections are rendered, using the data calculated by the game engine using simulation, and provided
     *  via the \c SimulationOutputs parameter.
     */
    ApplyReflections,

    /**
     *  **Type**: `FMOD_DSP_PARAMETER_TYPE_BOOL`
     *
     *  If true, pathing is rendered, using the data calculated by the game engine using simulation, and provided
     *  via the \c SimulationOutputs parameter.
     */
    ApplyPathing,

    /**
     *  **Type**: `FMOD_DSP_PARAMETER_TYPE_INT`
     *
     *  **Range**: 0 to 1.
     *
     *  Controls how HRTFs are interpolated when the source moves relative to the listener.
     *
     *  - `0`: Nearest-neighbor interpolation.
     *  - `1`: Bilinear interpolation.
     */
    HrtfInterpolation,

    /**
     *  **Type**: `FMOD_DSP_PARAMETER_TYPE_DATA`
     *
     *  DirectSoundPath from the Phonon DirectSimulator. It is recommended to update this every frame.
     */
    DirectSoundPath,

    /**
     *  **Type**: `FMOD_DSP_PARAMETER_TYPE_FLOAT`
     *
     *  **Range**: 0 to 1.
     *
     *  Blends between monopole (omnidirectional) and dipole directivity patterns. 0 = pure monopole (sound is emitted
     *  in all directions with equal intensity), 1 = pure dipole (sound is focused to the front and back of the source).
     *  At 0.5, the source has a cardioid directivity, with most of the sound emitted to the front of the source. Only
     *  used if \c ApplyDirectivity is set to \c 1.
     */
    DirectivityDipoleWeight,

    /**
     *  **Type**: `FMOD_DSP_PARAMETER_TYPE_FLOAT`
     *
     *  **Range**: 0 to 4.
     *
     *  Controls how focused the dipole directivity is. Higher values result in sharper directivity patterns. Only used
     *  if \c ApplyDirectivity is set to \c 1.
     */
    DirectivityDipolePower,

    /**
     *  **Type**: `FMOD_DSP_PARAMETER_TYPE_BOOL`
     *
     *  If true, applies HRTF-based 3D audio rendering to the direct sound path. Otherwise, sound is panned based on
     *  the speaker configuration.
     */
    DirectBinaural,
}

// Original copied from Steam Audio:
//// All parameters of the Phonon Spatializer which are available to users and FMOD internals.
// Any changes here must be reflected in the DSP description parameter list!
// #[derive(FromRepr, Debug, PartialEq)]
// #[repr(i32)]
// pub enum Params {
//     /**
//      *  **Type**: `FMOD_DSP_PARAMETER_TYPE_DATA`
//      *
//      *  World-space position of the source. Automatically written by FMOD Studio.
//      */
//     SourcePosition = 0,
//
//     /**
//      *  **Type**: `FMOD_DSP_PARAMETER_TYPE_DATA`
//      *
//      *  Overall linear gain of this effect. Automatically read by FMOD Studio.
//      */
//     OverallGain = 1,
//
//     /**
//      *  **Type**: `FMOD_DSP_PARAMETER_TYPE_INT`
//      *
//      *  **Range**: 0 to 2.
//      *
//      *  How to render distance attenuation.
//      *
//      *  -   `0`: Don't render distance attenuation.
//      *  -   `1`: Use a distance attenuation value calculated using the default physics-based model.
//      *  -   `2`: Use a distance attenuation value calculated using the curve specified in the FMOD Studio UI.
//      */
//     ApplyDistanceAttenuation = 2,
//
//     /**
//      *  **Type**: `FMOD_DSP_PARAMETER_TYPE_INT`
//      *
//      *  **Range**: 0 to 2.
//      *
//      *  How to render air absorption.
//      *
//      *  -   `0`: Don't render air absorption.
//      *  -   `1`: Use air absorption values calculated using the default exponential decay model.
//      *  -   `2`: Use air absorption values specified in the \c AirabsorptionLow, \c AIRABSORPTION_MID, and
//      *           \c AIRABSORPTION_HIGH parameters.
//      */
//     ApplyAirabsorption,
//
//     /**
//      *  **Type**: `FMOD_DSP_PARAMETER_TYPE_INT`
//      *
//      *  **Range**: 0 to 2.
//      *
//      *  How to render directivity.
//      *
//      *  -   `0`: Don't render directivity.
//      *  -   `1`: Use a directivity value calculated using the default dipole model, driven by the
//      *           \c DirectivityDipoleweight and \c DIRECTIVITY_DIPOLEPOWER parameters.
//      *  -   `2`: Use the directivity value specified in the \c DIRECTIVITY parameter.
//      */
//     ApplyDirectivity,
//
//     /**
//      *  **Type**: `FMOD_DSP_PARAMETER_TYPE_INT`
//      *
//      *  **Range**: 0 to 2.
//      *
//      *  How to render occlusion.
//      *
//      *  -   `0`: Don't render occlusion.
//      *  -   `1`: Use the occlusion value calculated by the game engine using simulation, and provided via the
//      *           \c SimulationOutputs parameter.
//      *  -   `2`: Use the occlusion value specified in the \c OCCLUSION parameter.
//      */
//     ApplyOcclusion,
//
//     /**
//      *  **Type**: `FMOD_DSP_PARAMETER_TYPE_INT`
//      *
//      *  **Range**: 0 to 2.
//      *
//      *  How to render transmission.
//      *
//      *  -   `0`: Don't render transmission.
//      *  -   `1`: Use the transmission values calculated by the game engine using simulation, and provided via the
//      *           \c SimulationOutputs parameter.
//      *  -   `2`: Use the transmission values specified in the \c TransmissionLow, \c TRANSMISSION_MID, and
//      *           \c TransmissionHigh parameters.
//      */
//     ApplyTransmission,
//
//     /**
//      *  **Type**: `FMOD_DSP_PARAMETER_TYPE_BOOL`
//      *
//      *  If true, reflections are rendered, using the data calculated by the game engine using simulation, and provided
//      *  via the \c SimulationOutputs parameter.
//      */
//     ApplyReflections,
//
//     /**
//      *  **Type**: `FMOD_DSP_PARAMETER_TYPE_BOOL`
//      *
//      *  If true, pathing is rendered, using the data calculated by the game engine using simulation, and provided
//      *  via the \c SimulationOutputs parameter.
//      */
//     ApplyPathing,
//
//     /**
//      *  **Type**: `FMOD_DSP_PARAMETER_TYPE_INT`
//      *
//      *  **Range**: 0 to 1.
//      *
//      *  Controls how HRTFs are interpolated when the source moves relative to the listener.
//      *
//      *  - `0`: Nearest-neighbor interpolation.
//      *  - `1`: Bilinear interpolation.
//      */
//     HrtfInterpolation,
//
//     /**
//      *  **Type**: `FMOD_DSP_PARAMETER_TYPE_FLOAT`
//      *
//      *  Not currently used.
//      */
//     DistanceAttenuation,
//
//     /*
//      *  **Type**: `FMOD_DSP_PARAMETER_TYPE_INT`
//      *
//      *  **Range**: 0 to 4.
//      *
//      *  Type of distance attenuation curve preset to use when \c APPLY_DISTANCEATTENUATION is \c 1.
//      *
//      *  - `0`: Linear squared rolloff.
//      *  - `1`: Linear rolloff.
//      *  - `2`: Inverse rolloff.
//      *  - `3`: Inverse squared rolloff.
//      *  - `4`: Custom rolloff.
//      */
//     DistanceAttenuationRolloffType,
//
//     /**
//      *  **Type**: `FMOD_DSP_PARAMETER_TYPE_FLOAT`
//      *
//      *  **Range**: 0 to 10000.
//      *
//      *  Minimum distance value for the distance attenuation curve.
//      */
//     DistanceAttenuationMinDistance,
//
//     /**
//      *  **Type**: `FMOD_DSP_PARAMETER_TYPE_FLOAT`
//      *
//      *  **Range**: 0 to 10000.
//      *
//      *  Maximum distance value for the distance attenuation curve.
//      */
//     DistanceAttenuationMaxDistance,
//
//     /**
//      *  **Type**: `FMOD_DSP_PARAMETER_TYPE_FLOAT`
//      *
//      *  **Range**: 0 to 1.
//      *
//      *  The low frequency (up to 800 Hz) EQ value for air absorption. Only used if \c ApplyAirabsorption is set to
//      *  \c 2. 0 = low frequencies are completely attenuated, 1 = low frequencies are not attenuated at all.
//      */
//     AirAbsorptionLow,
//
//     /**
//      *  **Type**: `FMOD_DSP_PARAMETER_TYPE_FLOAT`
//      *
//      *  **Range**: 0 to 1.
//      *
//      *  The middle frequency (800 Hz - 8 kHz) EQ value for air absorption. Only used if \c ApplyAirabsorption is set
//      *  to \c 2. 0 = middle frequencies are completely attenuated, 1 = middle frequencies are not attenuated at all.
//      */
//     AirAbsorptionMid,
//
//     /**
//      *  **Type**: `FMOD_DSP_PARAMETER_TYPE_FLOAT`
//      *
//      *  **Range**: 0 to 1.
//      *
//      *  The high frequency (8 kHz and above) EQ value for air absorption. Only used if \c ApplyAirabsorption is set to
//      *  \c 2. 0 = high frequencies are completely attenuated, 1 = high frequencies are not attenuated at all.
//      */
//     AirAbsorptionHigh,
//
//     /**
//      *  **Type**: `FMOD_DSP_PARAMETER_TYPE_FLOAT`
//      *
//      *  **Range**: 0 to 1.
//      *
//      *  The directivity attenuation value. Only used if \c ApplyDirectivity is set to \c 2. 0 = sound is completely
//      *  attenuated, 1 = sound is not attenuated at all.
//      */
//     Directivity,
//
//     /**
//      *  **Type**: `FMOD_DSP_PARAMETER_TYPE_FLOAT`
//      *
//      *  **Range**: 0 to 1.
//      *
//      *  Blends between monopole (omnidirectional) and dipole directivity patterns. 0 = pure monopole (sound is emitted
//      *  in all directions with equal intensity), 1 = pure dipole (sound is focused to the front and back of the source).
//      *  At 0.5, the source has a cardioid directivity, with most of the sound emitted to the front of the source. Only
//      *  used if \c ApplyDirectivity is set to \c 1.
//      */
//     DirectivityDipoleWeight,
//
//     /**
//      *  **Type**: `FMOD_DSP_PARAMETER_TYPE_FLOAT`
//      *
//      *  **Range**: 0 to 4.
//      *
//      *  Controls how focused the dipole directivity is. Higher values result in sharper directivity patterns. Only used
//      *  if \c ApplyDirectivity is set to \c 1.
//      */
//     DirectivityDipolePower,
//
//     /**
//      *  **Type**: `FMOD_DSP_PARAMETER_TYPE_FLOAT`
//      *
//      *  **Range**: 0 to 1.
//      *
//      *  The occlusion attenuation value. Only used if \c ApplyOcclusion is set to \c 2. 0 = sound is completely
//      *  attenuated, 1 = sound is not attenuated at all.
//      */
//     OCCLUSION,
//
//     /**
//      *  **Type**: `FMOD_DSP_PARAMETER_TYPE_INT`
//      *
//      *  **Range**: 0 to 1.
//      *
//      *  Specifies how the transmission filter is applied.
//      *
//      * - `0`: Transmission is modeled as a single attenuation factor.
//      * - `1`: Transmission is modeled as a 3-band EQ.
//      */
//     TransmissionType,
//
//     /**
//      *  **Type**: `FMOD_DSP_PARAMETER_TYPE_FLOAT`
//      *
//      *  **Range**: 0 to 1.
//      *
//      *  The low frequency (up to 800 Hz) EQ value for transmission. Only used if \c ApplyTransmission is set to \c 2.
//      *  0 = low frequencies are completely attenuated, 1 = low frequencies are not attenuated at all.
//      */
//     TransmissionLow,
//
//     /**
//      *  **Type**: `FMOD_DSP_PARAMETER_TYPE_FLOAT`
//      *
//      *  **Range**: 0 to 1.
//      *
//      *  The middle frequency (800 Hz to 8 kHz) EQ value for transmission. Only used if \c ApplyTransmission is set to
//      *  \c 2. 0 = middle frequencies are completely attenuated, 1 = middle frequencies are not attenuated at all.
//      */
//     TRANSMISSION_MID,
//
//     /**
//      *  **Type**: `FMOD_DSP_PARAMETER_TYPE_FLOAT`
//      *
//      *  **Range**: 0 to 1.
//      *
//      *  The high frequency (8 kHz and above) EQ value for transmission. Only used if \c ApplyTransmission is set to
//      *  \c 2. 0 = high frequencies are completely attenuated, 1 = high frequencies are not attenuated at all.
//      */
//     TransmissionHigh,
//
//     /**
//      *  **Type**: `FMOD_DSP_PARAMETER_TYPE_FLOAT`
//      *
//      *  **Range**: 0 to 1.
//      *
//      *  The contribution of the direct sound path to the overall mix for this event. Lower values reduce the
//      *  contribution more.
//      */
//     DirectMixLevel,
//
//     /**
//      *  **Type**: `FMOD_DSP_PARAMETER_TYPE_BOOL`
//      *
//      *  If true, applies HRTF-based 3D audio rendering to reflections. Results in an improvement in spatialization
//      *  quality when using convolution or hybrid reverb, at the cost of slightly increased CPU usage.
//      */
//     ReflectionsBinaural,
//
//     /**
//      *  **Type**: `FMOD_DSP_PARAMETER_TYPE_FLOAT`
//      *
//      *  **Range**: 0 to 10.
//      *
//      *  The contribution of reflections to the overall mix for this event. Lower values reduce the contribution more.
//      */
//     ReflectionsMixLevel,
//
//     /**
//      *  **Type**: `FMOD_DSP_PARAMETER_TYPE_BOOL`
//      *
//      *  If true, applies HRTF-based 3D audio rendering to pathing. Results in an improvement in spatialization
//      *  quality, at the cost of slightly increased CPU usage.
//      */
//     PathingBinaural,
//
//     /**
//      *  **Type**: `FMOD_DSP_PARAMETER_TYPE_FLOAT`
//      *
//      *  **Range**: 0 to 10.
//      *
//      *  The contribution of pathing to the overall mix for this event. Lower values reduce the contribution more.
//      */
//     PathingMixLevel,
//
//     /**
//      *  **Type**: `FMOD_DSP_PARAMETER_TYPE_DATA`
//      *
//      *  **DEPRECATED**
//      *
//      *  Pointer to the `IPLSimulationOutputs` structure containing simulation results.
//      */
//     SimulationOutputs,
//
//     /**
//      *  **Type**: `FMOD_DSP_PARAMETER_TYPE_BOOL`
//      *
//      *  If true, applies HRTF-based 3D audio rendering to the direct sound path. Otherwise, sound is panned based on
//      *  the speaker configuration.
//      */
//     DirectBinaural,
//
//     /**
//      *  **Type**: `FMOD_DSP_PARAMETER_TYPE_DATA`
//      *
//      *  (FMOD Studio 2.02+) The event's min/max distance range. Automatically set by FMOD Studio.
//      */
//     DistanceAttenuationRange,
//
//     /**
//      *  **Type**: `FMOD_DSP_PARAMETER_TYPE_INT`
//      *
//      *  Handle of the `IPLSource` object to use for obtaining simulation results. The handle can
//      *  be obtained by calling `iplFMODAddSource`.
//      */
//     SimulationOutputsHandle,
//
//     /**
//      *  **Type**: `FMOD_DSP_PARAMETER_TYPE_INT`
//      *
//      *  **Range**: 0 to 2.
//      *
//      *  Controls the output format.
//      *
//      *  - `0`: Output will be the format in FMOD's mixer.
//      *  - `1`: Output will be the format from FMOD's final output.
//      *  - `2`: Output will be the format from the event's input.
//      */
//     OutputFormat,
// }
