use firewheel::channel_config::{ChannelConfig, ChannelCount};
use firewheel::diff::{Diff, Patch, RealtimeClone};
use firewheel::event::ProcEvents;
use firewheel::node::{
    AudioNode, AudioNodeInfo, AudioNodeProcessor, ConstructProcessorContext, EmptyConfig,
    ProcBuffers, ProcExtra, ProcInfo, ProcessStatus,
};
use phonon::dsp::audio_buffer::{AudioBuffer, AudioSettings};
use phonon::effects::binaural::{BinauralEffect, BinauralEffectParameters};
use phonon::effects::direct::{DirectEffect, DirectEffectParameters};
use phonon::models::directivity::Directivity;
use phonon::simulators::direct::OcclusionType;

use crate::fixed_block::FixedProcessBlock;

#[derive(Diff, Patch, Debug, Clone, RealtimeClone, PartialEq, Default)]
#[cfg_attr(feature = "bevy", derive(bevy_ecs::prelude::Component))]
pub struct SpatializerNode {
    pub direct_effect_parameters: DirectEffectParameters,
    pub binaural_effect_parameters: BinauralEffectParameters,
    #[diff(skip)]
    /// Per-source simulator settings. It is not required to use these.
    pub simulator_settings: SimulatorSettings,
}

/// Per-source simulator settings
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SimulatorSettings {
    pub occlusion_type: OcclusionType,
    /// Size of the audio source when `OcclusionType` is set to `Volumetric`.
    pub occlusion_radius: f32,
    /// Number of occlusion samples to take when volumetric occlusion is enabled.
    /// Limited by `max_occlusion_samples` of the `DirectSimulator`.
    pub occlusion_samples: usize,
    // todo document what transmission is and what is needed to make it work (materials)
    pub num_transmission_rays: usize,
    pub hrtf_enable: bool, // todo, not used
    pub directivity: Directivity,
}

impl Default for SimulatorSettings {
    fn default() -> Self {
        SimulatorSettings {
            occlusion_type: OcclusionType::Volumetric,
            occlusion_radius: 1.0,
            occlusion_samples: 64,
            num_transmission_rays: 3,
            hrtf_enable: true,
            directivity: Directivity::default(),
        }
    }
}

impl AudioNode for SpatializerNode {
    // Since this node doesn't need any configuration, we'll just
    // default to `EmptyConfig`.
    type Configuration = EmptyConfig;

    // Return information about your node. This method is only ever called
    // once.
    fn info(&self, _config: &Self::Configuration) -> AudioNodeInfo {
        // The builder pattern is used for future-proofness as it is likely that
        // more fields will be added in the future.
        AudioNodeInfo::new()
            // A static name used for debugging purposes.
            .debug_name("spatializer_node")
            // The configuration of the input/output ports.
            .channel_config(ChannelConfig {
                num_inputs: ChannelCount::MONO,
                num_outputs: ChannelCount::STEREO,
            })
    }

    // Construct the realtime processor counterpart using the given information
    // about the audio stream.
    //
    // This method is called before the node processor is sent to the realtime
    // thread, so it is safe to do non-realtime things here like allocating.
    fn construct_processor(
        &self,
        _config: &Self::Configuration,
        cx: ConstructProcessorContext,
    ) -> impl AudioNodeProcessor {
        let sample_rate = cx.stream_info.sample_rate;
        let frame_size = 1024;

        let audio_settings = AudioSettings::new(sample_rate.get(), frame_size);
        let direct_effect = DirectEffect::new(audio_settings);
        let binaural_effect = BinauralEffect::new(audio_settings);

        Processor {
            direct_effect,
            binaural_effect,
            fixed_block: FixedProcessBlock::new(
                frame_size,
                cx.stream_info.max_block_frames.get() as usize,
                1,
                2,
            ),
            params: self.clone(),
            in_buf: AudioBuffer::new(frame_size),
            scratch_buf: AudioBuffer::new(frame_size),
            out_buf: AudioBuffer::new(frame_size),
        }
    }
}

// The realtime processor counterpart to your node.
struct Processor {
    params: SpatializerNode,
    fixed_block: FixedProcessBlock,
    direct_effect: DirectEffect,
    binaural_effect: BinauralEffect,
    in_buf: AudioBuffer<1>,
    scratch_buf: AudioBuffer<1>,
    out_buf: AudioBuffer<2>,
}

impl AudioNodeProcessor for Processor {
    // The realtime process method.
    fn process(
        &mut self,
        // Information about the process block.
        info: &ProcInfo,
        // The buffers of data to process.
        buffers: ProcBuffers,
        // The list of events for our node to process.
        events: &mut ProcEvents,
        // Extra buffers and utilities.
        _extra: &mut ProcExtra,
    ) -> ProcessStatus {
        for patch in events.drain_patches::<SpatializerNode>() {
            self.params.apply(patch);
        }

        // If the previous output of this node was silent, and the inputs are also silent
        // then we know there is no reverb tail left and we can skip processing.
        if info.prev_output_was_silent && info.in_silence_mask.all_channels_silent(1) {
            return ProcessStatus::ClearAllOutputs;
        }

        let temp_proc = ProcBuffers {
            inputs: buffers.inputs,
            outputs: buffers.outputs,
        };

        self.fixed_block
            .process(temp_proc, info, |inputs, outputs| {
                let direct_params = self.params.direct_effect_parameters;
                let binaural_params = self.params.binaural_effect_parameters;

                self.in_buf[0].copy_from_slice(inputs[0]);
                self.direct_effect
                    .apply(direct_params, &self.in_buf, &mut self.scratch_buf);
                self.binaural_effect
                    .apply(binaural_params, &self.scratch_buf, &mut self.out_buf);
                outputs[0].copy_from_slice(&self.out_buf[0]);
                outputs[1].copy_from_slice(&self.out_buf[1]);
            })
    }
}
