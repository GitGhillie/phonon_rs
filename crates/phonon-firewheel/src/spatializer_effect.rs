use firewheel::channel_config::{ChannelConfig, ChannelCount};
use firewheel::diff::{Diff, Patch, RealtimeClone};
use firewheel::event::ProcEvents;
use firewheel::node::{
    AudioNode, AudioNodeInfo, AudioNodeProcessor, ConstructProcessorContext, EmptyConfig,
    ProcBuffers, ProcExtra, ProcInfo, ProcessStatus,
};
use phonon::dsp::audio_buffer::{AudioBuffer, AudioSettings};
use phonon::effects::direct::{DirectEffect, DirectEffectParameters};

use crate::fixed_block::FixedProcessBlock;

#[derive(Diff, Patch, Debug, Clone, RealtimeClone, PartialEq, Default)]
pub struct SpatializerNode {
    /// Direct effect parameters
    pub direct_effect_parameters: DirectEffectParameters,
}

// Implement the AudioNode type for your node.
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
                num_outputs: ChannelCount::MONO,
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

        Processor {
            direct_effect,
            fixed_block: FixedProcessBlock::new(
                frame_size,
                cx.stream_info.max_block_frames.get() as usize,
                1,
                1,
            ),
            params: self.clone(),
            in_buf: AudioBuffer::new(frame_size),
            out_buf: AudioBuffer::new(frame_size),
        }
    }
}

// The realtime processor counterpart to your node.
struct Processor {
    params: SpatializerNode,
    fixed_block: FixedProcessBlock,
    direct_effect: DirectEffect,
    in_buf: AudioBuffer<1>,
    out_buf: AudioBuffer<1>,
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
                self.in_buf[0].copy_from_slice(inputs[0]);
                let direct_params = self.params.direct_effect_parameters;
                self.direct_effect
                    .apply(direct_params, &self.in_buf, &mut self.out_buf);
                outputs[0].copy_from_slice(&self.out_buf[0]);
            })
    }
}
