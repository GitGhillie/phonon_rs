use bitflags::bitflags;

use crate::direct_simulator::DirectSoundPath;

bitflags! {
    //todo check if these are all necessary
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    struct DirectApplyFlags: u8 {
        const DistanceAttenuation = 0b00001;
        const AirAbsorption = 0b00010;
        const Directivity = 0b00100;
        const Occlusion = 0b01000;
        const Transmission = 0b10000;
    }
}

enum TransmissionType {
    FrequencyIndependent,
    FrequencyDependent,
}

pub struct DirectEffectParams {
    direct_sound_path: DirectSoundPath,
    flags: DirectApplyFlags,
    transmission_type: TransmissionType,
}
