use eframe::egui::Context;
use eframe::{egui, Frame};
use kira::manager::backend::cpal::CpalBackend;
use kira::manager::{AudioManager, AudioManagerSettings};
use kira::sound::static_sound::{StaticSoundData, StaticSoundSettings};
use kira::track::TrackBuilder;
use phonon_kira::EqEffectBuilder;

fn main() {
    let eq_gains: [f32; 3] = [1.0, 0.5, 1.0];

    let mut manager = AudioManager::<CpalBackend>::new(AudioManagerSettings::default()).unwrap();

    let track = manager
        .add_sub_track(TrackBuilder::new().with_effect(EqEffectBuilder { eq_gains }))
        .unwrap();

    let sound_data = StaticSoundData::from_file(
        "data/audio/pink_noise.ogg",
        StaticSoundSettings::new().output_destination(&track),
    )
    .unwrap();

    manager.play(sound_data).unwrap();

    eframe::run_native(
        "EQ Effect Example (Kira)",
        eframe::NativeOptions::default(),
        Box::new(|cc| Box::<App>::default()),
    )
    .unwrap();
}

struct App {
    eq_gain0: f32,
    eq_gain1: f32,
    eq_gain2: f32,
}

impl Default for App {
    fn default() -> Self {
        Self {
            eq_gain0: 1.0,
            eq_gain1: 1.0,
            eq_gain2: 1.0,
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add(egui::Slider::new(&mut self.eq_gain0, 0.0..=1.0).text("Gain Low"));
            ui.add(egui::Slider::new(&mut self.eq_gain1, 0.0..=1.0).text("Gain Mid"));
            ui.add(egui::Slider::new(&mut self.eq_gain2, 0.0..=1.0).text("Gain High"));
        });
    }
}
