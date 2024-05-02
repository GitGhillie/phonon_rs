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

use crate::audio_buffer::{AudioBuffer, AudioEffectState};
use crate::speaker_layout::{SpeakerLayout, SpeakerLayoutType};
use glam::Vec3;
use std::f32::consts::PI;

/// Intermediate data for 2D pairwise constant-power panning.
#[derive(Default)]
struct PanningData {
    /// The two speaker indices we want to pan between.
    speaker_indices: [i32; 2],
    /// The angle between the speakers.
    angle_between_speakers: f32,
    /// The angle between the first speaker and the source.
    delta_phi: f32,
}

#[derive(Default)]
pub struct PanningEffectParameters {
    direction: Vec3,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PanningEffect {
    speaker_layout: SpeakerLayout,
    prev_direction: Vec3,
}

impl PanningEffect {
    pub fn new(layout: SpeakerLayoutType) -> Self {
        Self {
            speaker_layout: SpeakerLayout::new(layout),
            prev_direction: Vec3::ZERO,
        }
    }

    pub(crate) fn reset(&mut self) {
        self.prev_direction = Vec3::ZERO;
    }

    // todo: Support all channel layouts
    pub fn apply(
        &mut self,
        parameters: PanningEffectParameters,
        input: &AudioBuffer<1>,
        output: &mut AudioBuffer<2>,
    ) -> AudioEffectState {
        let panning_data = PanningData::default();
        let prev_panning_data = PanningData::default();

        // todo output.num_channels
        for i in 0..output.len() {
            let weight =
                Self::panning_weight(parameters.direction, &self.speaker_layout, i, &panning_data);
            let weight_prev = Self::panning_weight(
                self.prev_direction,
                &self.speaker_layout,
                i,
                &prev_panning_data,
            );

            // todo input.num_samples
            for j in 0..input[0].len() {
                // Crossfade between the panning coefficients for the previous frame and the
                // current frame.
                let alpha = (i as f32) / (input[0].len() as f32);
                let blended_weight = alpha * weight + (1.0 - alpha) * weight_prev;

                output[i][j] = blended_weight * input[0][j];
            }
        }

        self.prev_direction = parameters.direction;

        AudioEffectState::TailComplete
    }

    fn panning_weight(
        direction: Vec3,
        speaker_layout: &SpeakerLayout,
        index: usize,
        panning_data: &PanningData,
    ) -> f32 {
        match speaker_layout.layout_type {
            SpeakerLayoutType::Mono => 1.0,
            SpeakerLayoutType::Stereo => Self::stereo_panning_weight(direction, index),
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
        }
    }

    fn stereo_panning_weight(direction: Vec3, index: usize) -> f32 {
        let horizontal = direction.normalize();
        let p = horizontal.x;
        let q = (p + 1.0) * (PI / 4.0);

        if index == 0 {
            q.cos()
        } else {
            q.sin()
        }
    }
}
