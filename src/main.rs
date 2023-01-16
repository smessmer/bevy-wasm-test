use bevy::{
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    prelude::*,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(FrameTimeDiagnosticsPlugin)
        .add_startup_system(setup_camera)
        .add_startup_system(setup_fps_counter)
        .add_system(update_fps_counter)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

#[derive(Component)]
pub struct FpsDisplay;

pub fn setup_fps_counter(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(
            TextBundle::from_sections([TextSection {
                value: "".into(),
                style: TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 40.0,
                    color: Color::rgb(0.5, 0.0, 0.0),
                },
            }])
            .with_style(Style {
                position_type: PositionType::Absolute,
                align_self: AlignSelf::FlexStart,
                position: UiRect {
                    bottom: Val::Px(5.0),
                    left: Val::Px(15.0),
                    ..Default::default()
                },
                ..Default::default()
            }),
        )
        .insert(FpsDisplay);
}

pub fn update_fps_counter(
    diagnostics: Res<Diagnostics>,
    mut fps_displays: Query<&mut Text, With<FpsDisplay>>,
) {
    let fps = diagnostics
        .get(FrameTimeDiagnosticsPlugin::FPS)
        .expect("FrameTimeDiagnosticsPlugin is not set up");
    if let Some(fps_avg) = fps.average() {
        for mut fps_display in &mut fps_displays {
            fps_display.sections[0].value = format!("{fps_avg:.0}");
        }
    }
}
