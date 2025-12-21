use bevy::{
    diagnostic::FrameTimeDiagnosticsPlugin, input::common_conditions::input_just_pressed, prelude::*, window::{PrimaryWindow, WindowMode}
};
use bevy_egui::{EguiPlugin, EguiPrimaryContextPass};
use bevy_pancam::{PanCam, PanCamPlugin};
use config::{ConfigState, FormatableValue};
use glam::DVec2;
use palette::Palette;
use physics::{
    bodies::PointBody,
    forces::ForceMatrix,
    physics::ParticlePhysics,
};
use providers::positioners::*;
use rand::Rng as _;

mod config;
mod palette;
mod physics;
mod providers;
// mod snapshot;
mod ui;

const RADIUS: f32 = 0.5833334; // 0.5 * (0.5 + 2.0 / 3.0);

#[derive(States, PartialEq, Eq, Debug, Clone, Hash, Default)]
enum ShowUi {
    #[default]
    Yes,
    No,
}

#[derive(States, PartialEq, Eq, Debug, Clone, Hash, Default)]
enum AppState {
    #[default]
    Running,
    Paused,
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(0.025, 0.025, 0.025)))
        .add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        mode: WindowMode::BorderlessFullscreen(MonitorSelection::Primary),
                        ..default()
                    }),
                    ..default()
                }),
            EguiPlugin::default(),
            FrameTimeDiagnosticsPlugin::default(),
            PanCamPlugin::default(),
        ))
        .init_state::<AppState>()
        .init_state::<ShowUi>()
        .add_systems(Startup, setup)
        .add_systems(Update, (
            match_body_count,
            update_bodies.run_if(in_state(AppState::Running)),
            palette::update_palette,
            ui::toggle_running.run_if(input_just_pressed(KeyCode::Space)),
            ui::toggle_visible.run_if(input_just_pressed(KeyCode::Escape)),
        ))
        .add_systems(EguiPrimaryContextPass, ui::ui_system.run_if(in_state(ShowUi::Yes)))
        .run();
}

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    window: Single<&Window, With<PrimaryWindow>>,
) {

    let side = window.height().min(window.width());
    let mesh = meshes.add(Circle::new(RADIUS));
    let config = ConfigState {
        half_side: side * 0.5,
        body_mesh: Some(mesh.clone()),
        ..default()
    };
    let colors_count = config.colors_count.get_value();

    commands.insert_resource(ForceMatrix::new(colors_count, config.force_matrix_option));
    commands.insert_resource(Palette::new(&mut materials, config.colors_count.get_value()));
    commands.insert_resource(ParticlePhysics::default());
    commands.insert_resource(config);

    // let limit = side * (1.0 + 2.0 / 3.0);
    commands.spawn((
        Camera2d,
        PanCam {
            min_scale: 0.01,
            max_scale: 2.0,
            ..Default::default()
        },
    ));

}

fn match_body_count(
    mut commands: Commands,
    mut config: ResMut<ConfigState>,
    palette: Res<Palette>,
    query: Query<Entity, With<PointBody>>,
) {
    if config.reset_bodies {
        config.reset_bodies = false;
        for entity in query.iter() {
            commands.entity(entity).despawn();
        }
        for _ in 0..config.bodies_count.get_value() {
            spawn_particle(&mut commands, &config, &palette);
        }
    } else {
        let mut current_size = query.iter().count();
        let target_size = config.bodies_count.get_value();
        while current_size < target_size {
            spawn_particle(&mut commands, &config, &palette);
            current_size += 1;
        }
        let mut rng = rand::rng();
        while current_size > target_size {
            let rix = rng.random::<u64>() as usize % current_size;
            if let Some(entity) = query.iter().nth(rix) {
                commands.entity(entity).despawn();
                current_size -= 1;
            } else {
                panic!("stuck!");
            }
        }
    }
}

const CHILD_OFFSETS: [DVec2; 4] = [
    DVec2::new(1.5, 0.5), //right
    DVec2::new(0.5, 1.5), //top
    DVec2::new(-0.5, 0.5), // left
    DVec2::new(0.5, -0.5), // bottom
];

fn spawn_particle(commands: &mut Commands, config: &ConfigState, palette: &Palette) {
    let color = palette.random_ix();
    let body = PointBody::new(color, get_position(&config.position_option));
    let mut entity = commands.spawn((
        Mesh2d(config.body_mesh.clone().unwrap()),
        MeshMaterial2d(palette.get(color).clone()),
        get_offset_transform(vec3(&body.position), &config),
        body,
    ));
    entity.with_children(|commands| {
        for offset in &CHILD_OFFSETS {
            commands.spawn((
                Mesh2d(config.body_mesh.clone().unwrap()),
                MeshMaterial2d(palette.white().clone()),
                get_transform(vec3(offset), &config),
            ));
        }
    });
}

const DT: f64 = 1.0 / 60.0;

fn update_bodies(
    mut physics: ResMut<ParticlePhysics>,
    mut query: Query<(&mut Transform, &mut PointBody)>,
    config: Res<ConfigState>,
    force_matrix: Res<ForceMatrix>,
) {
    let bodies = query
        .iter()
        .map(|(_, body)| body)
        .collect::<Vec<_>>();
    let forces = physics.get_forces(&bodies, &force_matrix);
    query.iter_mut()
        .enumerate()
        .for_each(|(i, (mut transform, mut body))| {
            body.step(forces[i], DT);
            *transform = get_offset_transform(vec3(&body.position), &config);
        });
}

#[inline]
fn vec3(position: &DVec2) -> Vec3 {
    Vec3 {
        x: position.x as f32 * 2.0 - 1.0,
        y: position.y as f32 * 2.0 - 1.0,
        z: 0.0
    }
}

#[inline]
fn get_transform(pos: Vec3, config: &ConfigState) -> Transform {
    Transform::from_translation(pos * config.half_side)
}

#[inline]
fn get_offset_transform(pos: Vec3, config: &ConfigState) -> Transform {
    Transform::from_translation(pos * config.half_side + Vec3::new(config.panel_width * 0.75, 0.0, 0.0))
}
