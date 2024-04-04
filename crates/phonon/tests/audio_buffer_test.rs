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

use phonon::audio_buffer::AudioBuffer;

#[test]
fn mixing_audio_buffers() {
    let mut in1: AudioBuffer<1> = AudioBuffer::new(2);
    let mut in2: AudioBuffer<1> = AudioBuffer::new(2);
    let mut in3: AudioBuffer<1> = AudioBuffer::new(2);

    in1[0][0] = 1.0;
    in1[0][1] = 2.0;
    in2[0][0] = 3.0;
    in2[0][1] = 4.0;
    in3[0][0] = 7.0;
    in3[0][1] = 9.0;

    let mut out: AudioBuffer<1> = AudioBuffer::new(2);

    out.mix(&in1);
    out.mix(&in2);
    out.mix(&in3);

    assert_eq!(11.0, out[0][0]);
    assert_eq!(15.0, out[0][1]);

    out.make_silent();

    assert_eq!(0.0, out[0][0]);
}

#[test]
fn deinterleave() {
    let interleaved: [f32; 6] = [1.0, 2.0, 1.0, 2.0, 1.0, 2.0];

    let mut deinterleaved: AudioBuffer<2> = AudioBuffer::new(3);

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
    let mut deinterleaved: AudioBuffer<2> = AudioBuffer::new(2);

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
    let mut stereo: AudioBuffer<2> = AudioBuffer::new(2);

    stereo[0][0] = 1.0;
    stereo[0][1] = 1.0;
    stereo[1][0] = 2.0;
    stereo[1][1] = 2.0;

    let mut mono: AudioBuffer<1> = AudioBuffer::new(2);

    stereo.downmix(&mut mono);

    assert_eq!(1.5, mono[0][0]);
    assert_eq!(1.5, mono[0][1]);
}

// todo ambisonics test
// todo implement (tests for) other AudioBuffer functions
