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

use glam::Vec3;
use ndarray::Array1;

const MONO_SPEAKERS: [[f32; 3]; 1] = [[0.0, 0.0, 0.0]];

const STEREO_SPEAKERS: [[f32; 3]; 2] = [[-1.0, 0.0, 0.0], [1.0, 0.0, 0.0]];

#[derive(Debug, Clone, Copy)]
pub(crate) enum SpeakerLayoutType {
    Mono,
    Stereo,
    Quadraphonic,
    FivePointOne,
    SevenPointOne,
    Custom,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct SpeakerLayout {
    pub(crate) layout_type: SpeakerLayoutType,
    num_speakers: i16,
    speakers: Array1<Vec3>,
}

impl SpeakerLayout {
    pub fn new(layout_type: SpeakerLayoutType) -> Self {
        Self {
            layout_type,
            num_speakers: Self::num_speakers_for_layout(layout_type),
            speakers: match layout_type {
                SpeakerLayoutType::Mono => {
                    Vec::from_iter(MONO_SPEAKERS.iter().map(|x| Vec3::new(x[0], x[1], x[2])))
                }
                SpeakerLayoutType::Stereo => {
                    Vec::from_iter(STEREO_SPEAKERS.iter().map(|x| Vec3::new(x[0], x[1], x[2])))
                }
                SpeakerLayoutType::Quadraphonic => {
                    todo!()
                }
                SpeakerLayoutType::FivePointOne => {
                    todo!()
                }
                SpeakerLayoutType::SevenPointOne => {
                    todo!()
                }
                SpeakerLayoutType::Custom => {
                    todo!()
                }
            },
        }
    }

    fn num_speakers_for_layout(layout_type: SpeakerLayoutType) -> i16 {
        match layout_type {
            SpeakerLayoutType::Mono => 1,
            SpeakerLayoutType::Stereo => 2,
            SpeakerLayoutType::Quadraphonic => 4,
            SpeakerLayoutType::FivePointOne => 6,
            SpeakerLayoutType::SevenPointOne => 8,
            SpeakerLayoutType::Custom => {
                unimplemented!()
            }
        }
    }
}
