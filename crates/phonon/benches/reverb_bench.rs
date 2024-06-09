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
use phonon::reverb_effect::{ReverbEffect, ReverbEffectParams};
use phonon::reverb_estimator::Reverb;

fn bench_reverb(input: f32) -> f32 {
    const NUM_RUNS: i32 = 100_000; //todo this should be a Criterion argument probably
    const SAMPLING_RATE: i32 = 48_000;
    const FRAME_SIZE: usize = 1024;

    // let in_array = [0.0f32; FRAME_SIZE];
    // let out_array = [0.0f32; FRAME_SIZE];

    let mut in_buffer: AudioBuffer<1> = AudioBuffer::new(FRAME_SIZE);
    let mut out_buffer: AudioBuffer<1> = AudioBuffer::new(FRAME_SIZE);
    //todo fill in_array/in_buffer with random data

    let audio_settings = AudioSettings::new(SAMPLING_RATE, FRAME_SIZE);

    let reverb_effect = ReverbEffect::new(audio_settings);

    let reverb = Reverb {
        reverb_times: [2.0, 1.5, 1.0],
    };

    // todo start timer
    // todo x runs

    // let reverb_params = ReverbEffectParams(reverb);
    // reverb_effect.apply(reverb_params, in_buffer, out_buffer);

    // todo get time

    0.0
}

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("mix 20", |b| b.iter(|| bench_reverb(black_box(20.0))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
