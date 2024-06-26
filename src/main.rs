use bevy::prelude::*;
use iyes_perf_ui::{PerfUiPlugin, PerfUiRoot};
use iyes_perf_ui::diagnostics::PerfUiEntryFPS;
use iyes_perf_ui::prelude::PerfUiEntryFPSWorst;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(bevy::diagnostic::FrameTimeDiagnosticsPlugin)
        .add_plugins(PerfUiPlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn((
        PerfUiRoot {
            display_labels: true,
            layout_horizontal: false,
            ..default()
        },
        PerfUiEntryFPSWorst::default(),
        PerfUiEntryFPS::default(),
    ));
}