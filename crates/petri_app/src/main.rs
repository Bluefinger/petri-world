use bevy::{diagnostic::LogDiagnosticsPlugin, prelude::*};
use petri_simulation as sim;
use sim::SimulationPlugin;

fn main() {
    App::build()
        .insert_resource(ClearColor(Color::MIDNIGHT_BLUE))
        .insert_resource(WindowDescriptor {
            title: "Petri World!".to_string(),
            width: 800.0,
            height: 800.0,
            ..Default::default()
        })
        .insert_resource(Msaa { samples: 4 })
        .add_plugin(LogDiagnosticsPlugin::default())
        // .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugins(DefaultPlugins)
        .add_plugin(SimulationPlugin)
        .run();
}
