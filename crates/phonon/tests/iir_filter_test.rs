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

use phonon::dsp::iir::{IIRFilterer, IIR};

#[test]
fn iir_filter() {
    let coefficients: [f32; 5] = [2.0, 3.0, 4.0, 5.0, 6.0];

    let filter = IIR::new_from_coefficients(coefficients);
    let mut filterer = IIRFilterer::new(filter);

    let dry: [f32; 5] = [1.0, 2.0, 3.0, 4.0, 5.0];
    let mut wet: [f32; 5] = [0.0; 5];

    filterer.apply(dry.len(), dry.as_slice(), wet.as_mut_slice());

    assert_eq!([4.0, 5.0, 6.0, 16.0, 8.0], wet);
}
