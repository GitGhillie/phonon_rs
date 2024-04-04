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

use biquad::*;

#[test]
fn iir_filter() {
    let coefficients = Coefficients::<f32> {
        a1: 2.0,
        a2: 3.0,
        b0: 4.0,
        b1: 5.0,
        b2: 6.0,
    };

    let mut biquad1 = DirectForm1::<f32>::new(coefficients);

    let dry = [1.0, 2.0, 3.0, 4.0, 5.0];
    let mut wet: [f32; 5] = [0.0; 5];

    for i in 0..dry.len() {
        wet[i] = biquad1.run(dry[i]);
    }

    assert_eq!([4.0, 5.0, 6.0, 16.0, 8.0], wet);
}
