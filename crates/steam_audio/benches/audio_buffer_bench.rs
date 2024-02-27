use criterion::{black_box, criterion_group, criterion_main, Criterion};
use steam_audio::audio_buffer::AudioBuffer;

// Todo: fill input with arrays of random numbers
fn mix_buffers(input: f32) -> f32 {
    let mut in1: AudioBuffer<1, 200> = AudioBuffer::new();
    let mut in2: AudioBuffer<1, 200> = AudioBuffer::new();
    let mut in3: AudioBuffer<1, 200> = AudioBuffer::new();

    in1[0][0] = 1.0;
    in1[0][1] = 2.0;
    in2[0][0] = input;
    in2[0][1] = 4.0;
    in3[0][0] = 7.0;
    in3[0][1] = 9.0;

    let mut out: AudioBuffer<1, 200> = AudioBuffer::new();

    out.mix(&in1);
    out.mix(&in2);
    out.mix(&in3);

    out[0][1]
}

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("mix 20", |b| b.iter(|| mix_buffers(black_box(20.0))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
