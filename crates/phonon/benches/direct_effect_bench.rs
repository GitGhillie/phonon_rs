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

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use phonon::audio_buffer::{AudioBuffer, AudioSettings};
use phonon::direct_effect::{
    DirectApplyFlags, DirectEffect, DirectEffectParameters, TransmissionType,
};
use phonon::direct_simulator::DirectSoundPath;
use rand::Rng;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("direct effect", |b| {
        let apply_transmission = true;
        let transmission_type = TransmissionType::FrequencyDependent;
        let _num_channels = 1;

        let sampling_rate = 48_000;
        let frame_size = 1024;

        let render_settings = AudioSettings::new(sampling_rate, frame_size);

        let mut direct_effect = DirectEffect::new(render_settings);

        let mut in_buffer = AudioBuffer::new(frame_size);
        let mut out_buffer = AudioBuffer::new(frame_size);

        let mut rng = rand::thread_rng();
        for sample in &mut in_buffer[0] {
            let random_sample = (rng.gen_range(0..i32::MAX) % 10001) as f32 / 10000.0f32;
            *sample = black_box(random_sample);
        }

        let mut direct_params = DirectEffectParameters {
            direct_sound_path: DirectSoundPath {
                distance_attenuation: 1.0,
                air_absorption: [0.1, 0.2, 0.3],
                delay: 0.0,
                occlusion: 0.5,
                transmission: [0.1, 0.2, 0.3],
                directivity: 0.0,
            },
            flags: DirectApplyFlags::DistanceAttenuation | DirectApplyFlags::Occlusion,
            transmission_type,
        };

        direct_params
            .flags
            .set(DirectApplyFlags::Transmission, apply_transmission);

        b.iter(|| {
            // Changing transmission factor each run to get the worst case performance.
            direct_params.direct_sound_path.transmission[0] =
                (black_box(0.1) + black_box(0.1)) / 100.0;
            direct_effect.apply(direct_params, &in_buffer, &mut out_buffer);
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
