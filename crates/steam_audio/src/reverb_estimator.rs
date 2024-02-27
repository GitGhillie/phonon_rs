use crate::bands::NUM_BANDS;

pub struct Reverb {
    pub reverb_times: [f32; NUM_BANDS],
}

impl Default for Reverb {
    fn default() -> Self {
        Reverb {
            reverb_times: [0.1; NUM_BANDS],
        }
    }
}
