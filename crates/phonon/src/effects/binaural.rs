use crate::dsp::audio_buffer::{AudioBuffer, AudioEffectState, AudioSettings};
use glam::Vec3;
use sofar::reader::{OpenOptions, Sofar};
use sofar::render::Renderer;

#[derive(Debug, Copy, Clone)]
pub struct BinauralEffectParameters {
    // todo check if the following is still correct
    /// Direction relative to the listener. Will be normalized by the BinauralEffect.
    pub direction: Vec3,
}

impl Default for BinauralEffectParameters {
    fn default() -> Self {
        Self {
            direction: Vec3::new(0.0, 1.0, 0.0),
        }
    }
}

pub struct BinauralEffect {
    renderer: Renderer,
    sofa: Sofar,
    filter: sofar::reader::Filter,
}

impl BinauralEffect {
    pub fn new(audio_settings: AudioSettings) -> Self {
        let sampling_rate = audio_settings.sampling_rate;
        let frame_size = audio_settings.frame_size;

        let sofa = OpenOptions::new()
            .sample_rate(sampling_rate as f32)
            .open("data/hrtf/cipic_124.sofa") // todo
            .unwrap();

        let filter_len = sofa.filter_len();
        let filter = sofar::reader::Filter::new(filter_len);

        let renderer = Renderer::builder(filter_len)
            .with_sample_rate(sampling_rate as f32)
            .with_partition_len(frame_size)
            .build()
            .unwrap();

        Self {
            renderer,
            sofa,
            filter,
        }
    }

    pub fn apply(
        &mut self,
        params: BinauralEffectParameters,
        input: &AudioBuffer<1>,
        output: &mut AudioBuffer<2>,
    ) -> AudioEffectState {
        // todo check if input and output are equal in num samples?

        // todo silence output?

        // todo: Currently panics if direction is 0, 0, 0.
        let dir = params.direction;
        self.sofa.filter(dir.x, dir.y, dir.z, &mut self.filter);
        self.renderer.set_filter(&self.filter).unwrap();

        let input_data: &[f32] = &input.0[0];
        let [left_channel, right_channel] = &mut output.0;

        self.renderer
            .process_block(
                input_data,
                left_channel.as_mut_slice(),
                right_channel.as_mut_slice(),
            )
            .unwrap();

        AudioEffectState::TailComplete
    }
}
