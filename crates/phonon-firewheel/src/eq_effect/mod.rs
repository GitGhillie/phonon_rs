use firewheel::Volume;
use firewheel::channel_config::{ChannelConfig, ChannelCount};
use firewheel::diff::{Diff, Patch, RealtimeClone};
use firewheel::dsp::volume::DEFAULT_AMP_EPSILON;
use firewheel::event::ProcEvents;
use firewheel::node::{
    AudioNode, AudioNodeInfo, AudioNodeProcessor, ConstructProcessorContext, EmptyConfig,
    ProcBuffers, ProcExtra, ProcInfo, ProcessStatus,
};
use phonon::dsp::audio_buffer::{AudioBuffer, AudioSettings};
use phonon::effects::eq::{EqEffect, EqEffectParameters};

use crate::fixed_block::FixedProcessBlock;

#[derive(Diff, Patch, Debug, Clone, RealtimeClone, PartialEq)]
pub struct FilterNode {
    /// The overall volume.
    pub volume: Volume,
    /// EQ bands
    pub eq: [f32; 3],
}

impl Default for FilterNode {
    fn default() -> Self {
        Self {
            volume: Volume::default(),
            eq: [1.0, 1.0, 1.0],
        }
    }
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
        let eq_effect = EqEffect::new(audio_settings);

        Processor {
            gain: 1.0,
            eq_effect,
            fixed_block: FixedProcessBlock::new(
                frame_size as usize,
                cx.stream_info.max_block_frames.get() as usize,
                2,
                2,
            ),
            phonon_buffer_in_l: AudioBuffer::new(frame_size),
            phonon_buffer_in_r: AudioBuffer::new(frame_size),
            phonon_buffer_out_l: AudioBuffer::new(frame_size),
            phonon_buffer_out_r: AudioBuffer::new(frame_size),
            eq: [1.0, 1.0, 1.0],
        }
    }
}

// The realtime processor counterpart to your node.
struct Processor {
    fixed_block: FixedProcessBlock,
    phonon_buffer_in_l: AudioBuffer<1>,
    phonon_buffer_in_r: AudioBuffer<1>,
    phonon_buffer_out_l: AudioBuffer<1>,
    phonon_buffer_out_r: AudioBuffer<1>,
    gain: f32,
    eq: [f32; 3],
    eq_effect: EqEffect,
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
        // Process the events.
        //
        // We don't need to keep around a `FilterNode` instance,
        // so we can just match on each event directly.
        for patch in events.drain_patches::<FilterNode>() {
            match patch {
                FilterNodePatch::Volume(volume) => {
                    self.gain = volume.amp_clamped(DEFAULT_AMP_EPSILON);
                }
                FilterNodePatch::Eq(eq_event) => {
                    self.eq[eq_event.0] = eq_event.1;
                }
            }
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
        let result = self
            .fixed_block
            .process(temp_proc, info, |inputs, outputs| {
                // todo: EqEffectParameters as node param?
                let eq_params = EqEffectParameters { gains: self.eq };

                self.phonon_buffer_in_l.0[0] = inputs[0].to_vec();
                self.phonon_buffer_in_r.0[0] = inputs[1].to_vec();
                // todo make it easier to apply this to multiple channels
                self.eq_effect.apply(
                    eq_params,
                    &self.phonon_buffer_in_l,
                    &mut self.phonon_buffer_out_l,
                );
                self.eq_effect.apply(
                    eq_params,
                    &self.phonon_buffer_in_r,
                    &mut self.phonon_buffer_out_r,
                );

                outputs[0].copy_from_slice(self.phonon_buffer_out_l.0[0].as_slice());
                outputs[1].copy_from_slice(self.phonon_buffer_out_r.0[0].as_slice());

                for i in 0..inputs[0].len() {
                    outputs[0][i] = inputs[0][i] * self.gain;
                    outputs[1][i] = inputs[1][i] * self.gain;
                }
            });

        result
    }
}
