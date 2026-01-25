use std::hint::black_box;

use criterion::{Criterion, criterion_group, criterion_main};
use phonon::dsp::audio_buffer::{AudioBuffer, ScratchBuffer};

fn mix_buffers(in1: ScratchBuffer) -> ScratchBuffer {
    // Note that you normally would not create buffers in the audio hot path
    let mut in2 = ScratchBuffer::new(1, 200);
    let mut in3 = ScratchBuffer::new(1, 200);

    in2[0][0] = 3.0;
    in2[0][1] = 4.0;
    in3[0][0] = 7.0;
    in3[0][1] = 9.0;

    let mut out = ScratchBuffer::new(1, 200);
    let out_ref = &mut out.as_ref_mut();

    in1.as_ref().mix(out_ref);
    in2.as_ref().mix(out_ref);
    in3.as_ref().mix(out_ref);

    out
}

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("mix buffers", |b| {
        b.iter(|| {
            let mut buf = ScratchBuffer::new(1, 200);
            buf[0][5] = black_box(0.0);
            mix_buffers(black_box(buf))
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
