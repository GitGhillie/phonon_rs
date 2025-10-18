use firewheel::channel_config::{ChannelConfig, ChannelCount};
use firewheel::diff::{Diff, Patch, RealtimeClone};
use firewheel::event::ProcEvents;
use firewheel::node::{
    AudioNode, AudioNodeInfo, AudioNodeProcessor, ConstructProcessorContext, EmptyConfig,
    ProcBuffers, ProcExtra, ProcInfo, ProcessStatus,
};
use phonon::dsp::audio_buffer::AudioSettings;
use phonon::effects::eq::{EqEffect, EqEffectParameters};

use crate::fixed_block::FixedProcessBlock;

#[derive(Diff, Patch, Debug, Clone, RealtimeClone, PartialEq, Default)]
pub struct FilterNode {
    /// EQ effect parameters
    pub eq_effect_parameters: EqEffectParameters,
}

// Implement the AudioNode type for your node.
impl AudioNode for FilterNode {
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
            .debug_name("example_filter_")
            // The configuration of the input/output ports.
            .channel_config(ChannelConfig {
                num_inputs: ChannelCount::STEREO,
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
        let eq_effect_l = EqEffect::new(audio_settings);
        let eq_effect_r = EqEffect::new(audio_settings);

        Processor {
            eq_effect_l,
            eq_effect_r,
            fixed_block: FixedProcessBlock::new(
                frame_size,
                cx.stream_info.max_block_frames.get() as usize,
                2,
                2,
            ),
            params: self.clone(),
        }
    }
}

// The realtime processor counterpart to your node.
struct Processor {
    params: FilterNode,
    fixed_block: FixedProcessBlock,
    eq_effect_l: EqEffect,
    eq_effect_r: EqEffect,
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
        for patch in events.drain_patches::<FilterNode>() {
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
                let eq_params = self.params.eq_effect_parameters;

                self.eq_effect_l.apply(eq_params, inputs[0], outputs[0]);
                self.eq_effect_r.apply(eq_params, inputs[1], outputs[1]);
            })
    }
}
