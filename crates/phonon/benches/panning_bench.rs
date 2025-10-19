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

// Port note: Steam Audio measures the result in terms of Max Sources.
// To get the time per run, do ( 1024 / ( 48000 * Max Sources )

use criterion::{Criterion, black_box, criterion_group, criterion_main};
use glam::Vec3;
use phonon::dsp::audio_buffer::AudioBuffer;
use phonon::dsp::speaker_layout::SpeakerLayoutType;
use phonon::effects::panning::{PanningEffect, PanningEffectParameters};
use rand::Rng;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("panning effect", |b| {
        let frame_size = 1024;

        let mut in_buffer = AudioBuffer::new(frame_size);
        let mut out_buffer = AudioBuffer::new(frame_size);

        let mut rng = rand::thread_rng();
        for sample in &mut in_buffer[0] {
            let random_sample = (rng.gen_range(0..i32::MAX) % 10001) as f32 / 10000.0f32;
            *sample = black_box(random_sample);
        }

        let mut panning_effect = PanningEffect::new(SpeakerLayoutType::Stereo);
        let direction = Vec3::new(1.0, 0.0, 0.0);

        b.iter(|| {
            let panning_params = PanningEffectParameters { direction };
            panning_effect.apply(panning_params, &in_buffer, &mut out_buffer);
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
