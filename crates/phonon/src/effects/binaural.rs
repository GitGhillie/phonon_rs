use crate::dsp::audio_buffer::{AudioBuffer, AudioEffectState, AudioSettings};
use glam::Vec3;
use sofar::reader::{OpenOptions, Sofar};
use sofar::render::Renderer;

#[derive(Debug, Copy, Clone)]
pub struct BinauralEffectParameters {
    /// Direction/position relative to the listener. Should not be normalized.
    /// Avoid going through 0.0, 0.0, 0.0, as this will result in a jarring change in the audio.
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
    /// Position relative to the listener. Should not be normalized.
    /// This may never reach Vec3::ZERO, as that will result in a panic.
    direction: Vec3,
}

impl BinauralEffect {
    // todo: Check if a renderer can be shared between multiple instances of the effect.
    pub fn new(audio_settings: AudioSettings) -> Self {
        let sampling_rate = audio_settings.sampling_rate;
        let frame_size = audio_settings.frame_size;

        let sofa_bytes = include_bytes!("../../data/hrtf/cipic_124.sofa");

        let sofa = OpenOptions::new()
            .sample_rate(sampling_rate as f32)
            .open_data(sofa_bytes)
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
            direction: Vec3::new(0.0, 1.0, 0.0),
        }
    }

    pub fn apply(
        &mut self,
        params: BinauralEffectParameters,
        input: &AudioBuffer<1>,
        output: &mut AudioBuffer<2>,
    ) -> AudioEffectState {
        // todo silence output?

        if params.direction != Vec3::ZERO {
            self.direction = params.direction;
        }

        let dir = self.direction;
        self.sofa.filter(dir.y, -dir.x, dir.z, &mut self.filter);
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
