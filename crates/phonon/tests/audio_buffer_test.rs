use steam_audio::audio_buffer::AudioBuffer;

#[test]
fn mixing_audio_buffers() {
    let mut in1: AudioBuffer<1, 2> = AudioBuffer::new();
    let mut in2: AudioBuffer<1, 2> = AudioBuffer::new();
    let mut in3: AudioBuffer<1, 2> = AudioBuffer::new();

    in1[0][0] = 1.0;
    in1[0][1] = 2.0;
    in2[0][0] = 3.0;
    in2[0][1] = 4.0;
    in3[0][0] = 7.0;
    in3[0][1] = 9.0;

    let mut out: AudioBuffer<1, 2> = AudioBuffer::new();

    //todo make silent function

    out.mix(&in1);
    out.mix(&in2);
    out.mix(&in3);

    assert_eq!(11.0, out[0][0]);
    assert_eq!(15.0, out[0][1]);
}

#[test]
fn deinterleave() {
    let interleaved: [f32; 6] = [1.0, 2.0, 1.0, 2.0, 1.0, 2.0];

    let mut deinterleaved: AudioBuffer<2, 3> = AudioBuffer::new();

    deinterleaved.write(&interleaved);

    assert_eq!(1.0, deinterleaved[0][0]);
    assert_eq!(1.0, deinterleaved[0][1]);
    assert_eq!(1.0, deinterleaved[0][2]);
    assert_eq!(2.0, deinterleaved[1][0]);
    assert_eq!(2.0, deinterleaved[1][1]);
    assert_eq!(2.0, deinterleaved[1][2]);
}

#[test]
fn interleave() {
    let mut deinterleaved: AudioBuffer<2, 2> = AudioBuffer::new();

    deinterleaved[0][0] = 1.0;
    deinterleaved[0][1] = 1.0;
    deinterleaved[1][0] = 2.0;
    deinterleaved[1][1] = 2.0;

    let mut interleaved: [f32; 4] = [0.0; 4];

    deinterleaved.read(&mut interleaved);

    assert_eq!(1.0, interleaved[0]);
    assert_eq!(2.0, interleaved[1]);
    assert_eq!(1.0, interleaved[2]);
    assert_eq!(2.0, interleaved[3]);
}

#[test]
fn downmix_to_mono() {
    let mut stereo: AudioBuffer<2, 2> = AudioBuffer::new();

    stereo[0][0] = 1.0;
    stereo[0][1] = 1.0;
    stereo[1][0] = 2.0;
    stereo[1][1] = 2.0;

    let mut mono: AudioBuffer<1, 2> = AudioBuffer::new();

    stereo.downmix(&mut mono);

    assert_eq!(1.5, mono[0][0]);
    assert_eq!(1.5, mono[0][1]);
}

// todo ambisonics test
// todo implement (tests for) other AudioBuffer functions
