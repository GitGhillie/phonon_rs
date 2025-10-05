use eframe::egui;
use firewheel::FirewheelContext;
use firewheel::diff::Memo;
use firewheel::error::UpdateError;
use firewheel::node::NodeID;
use firewheel::nodes::sampler::{RepeatMode, SamplerNode};
use phonon_firewheel::eq_effect::FilterNode;
use symphonium::SymphoniumLoader;

struct AudioSystem {
    cx: FirewheelContext,

    _sampler_node: SamplerNode,
    _sampler_node_id: NodeID,

    eq_node: Memo<FilterNode>,
    eq_node_id: NodeID,
}

impl AudioSystem {
    fn new() -> Self {
        let mut cx = FirewheelContext::new(Default::default());
        cx.start_stream(Default::default()).unwrap();

        let sample_rate = cx.stream_info().unwrap().sample_rate;

        let mut loader = SymphoniumLoader::new();
        let sample = firewheel::load_audio_file(
            &mut loader,
            "data/audio/windless_slopes.ogg",
            sample_rate,
            Default::default(),
        )
        .unwrap()
        .into_dyn_resource();

        let graph_out_node_id = cx.graph_out_node_id();

        let mut sampler_node = SamplerNode::default();
        sampler_node.set_sample(sample);
        sampler_node.repeat_mode = RepeatMode::RepeatEndlessly;
        sampler_node.start_or_restart();

        let eq_node = FilterNode::default();

        let sampler_node_id = cx.add_node(sampler_node.clone(), None);
        let eq_node_id = cx.add_node(eq_node.clone(), None);

        cx.connect(sampler_node_id, eq_node_id, &[(0, 0), (1, 1)], false)
            .unwrap();
        cx.connect(eq_node_id, graph_out_node_id, &[(0, 0), (1, 1)], false)
            .unwrap();

        Self {
            cx,
            _sampler_node: sampler_node,
            _sampler_node_id: sampler_node_id,
            eq_node: Memo::new(eq_node),
            eq_node_id,
        }
    }

    fn update(&mut self) {
        if let Err(e) = self.cx.update() {
            println!("{:?}", &e);

            if let UpdateError::StreamStoppedUnexpectedly(_) = e {
                // The stream has stopped unexpectedly (i.e the user has
                // unplugged their headphones.)
                //
                // Typically you should start a new stream as soon as
                // possible to resume processing (event if it's a dummy
                // output device).
                //
                // In this example we just quit the application.
                panic!("Stream stopped unexpectedly!");
            }
        }
    }
}

fn main() {
    let mut audio_system = AudioSystem::new();
    let mut gain: f32 = 1.0;

    eframe::run_simple_native(
        "EQ & Gain Effect (Kira)",
        eframe::NativeOptions::default(),
        move |ctx, _frame| {
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.add(egui::Slider::new(&mut gain, 0.0..=1.0).text("Gain"));
            });
            audio_system.eq_node.volume = firewheel::Volume::Linear(gain);
            audio_system
                .eq_node
                .update_memo(&mut audio_system.cx.event_queue(audio_system.eq_node_id));
            audio_system.update();
        },
    )
    .unwrap()
}
