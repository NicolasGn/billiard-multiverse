use bevy::{prelude::*, window::WindowResolution};

use crate::{billiard::Ball, simulation::Simulation};

#[derive(Component)]
struct BallComponent {
    n: usize,
    billiard_id: u8,
}

pub struct VisualizationPlugin {
    size: f32,
}

impl VisualizationPlugin {
    pub const fn init(size: f32) -> Self {
        Self { size }
    }
}

impl Plugin for VisualizationPlugin {
    fn build(&self, app: &mut App) {
        let window_plugin = WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(self.size, self.size)
                    .with_scale_factor_override(1.0),
                resizable: false,
                ..default()
            }),
            ..default()
        };

        app.add_plugins((
            TransformPlugin,
            HierarchyPlugin,
            bevy::input::InputPlugin,
            bevy::a11y::AccessibilityPlugin,
            window_plugin,
            bevy::asset::AssetPlugin::default(),
            bevy::winit::WinitPlugin::<bevy::winit::WakeUp>::default(),
            bevy::render::RenderPlugin::default(),
            ImagePlugin::default(),
            bevy::render::pipelined_rendering::PipelinedRenderingPlugin,
            bevy::core_pipeline::CorePipelinePlugin,
            bevy::sprite::SpritePlugin::default(),
            bevy::text::TextPlugin,
            DefaultPickingPlugins,
        ));

        app.add_systems(Startup, (spawn_camera, spawn_space, spawn_balls));

        app.add_systems(PreUpdate, handle_inputs);

        app.add_systems(PostUpdate, render_balls);
    }
}

fn spawn_camera(mut commands: Commands, simulation: Res<Simulation>) {
    let size = simulation.size() as f32;
    commands.spawn((Transform::from_xyz(size / 2.0, size / 2.0, 0.0), Camera2d));
}

fn spawn_space(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    simulation: Res<Simulation>,
) {
    let size = simulation.size() as f32;

    let rectangle = Rectangle::from_length(size as f32);
    let green = Color::linear_rgb(0.01, 0.01, 0.01);

    commands.spawn((
        Transform::from_xyz(size / 2.0, size / 2.0, 0.0),
        Mesh2d(meshes.add(rectangle)),
        MeshMaterial2d(materials.add(green)),
    ));
}

fn spawn_balls(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    simulation: Res<Simulation>,
) {
    spawn_billiard_balls(
        1,
        simulation.balls(1),
        Color::linear_rgb(0.0, 0.0, 1.0),
        1.0,
        &mut commands,
        &mut meshes,
        &mut materials,
    );

    spawn_billiard_balls(
        2,
        simulation.balls(2),
        Color::linear_rgb(1.0, 0.0, 0.0),
        2.0,
        &mut commands,
        &mut meshes,
        &mut materials,
    );
}

fn spawn_billiard_balls(
    billiard_id: u8,
    balls: &Vec<Ball>,
    color: Color,
    z: f32,
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<ColorMaterial>,
) {
    for ball in balls.iter() {
        commands
            .spawn((
                Transform::from_xyz(ball.position.x as f32, ball.position.y as f32, z),
                Mesh2d(meshes.add(Annulus::new((ball.radius - 1.0) as f32, ball.radius as f32))),
                MeshMaterial2d(materials.add(color)),
                BallComponent {
                    n: ball.n,
                    billiard_id,
                },
            ))
            .with_children(|parent| {
                parent.spawn((
                    Transform::from_xyz(0.0, (ball.radius / 2.0) as f32, 0.0),
                    Mesh2d(meshes.add(Rectangle::new(1.0, ball.radius as f32))),
                    MeshMaterial2d(materials.add(color)),
                ));
                parent.spawn((
                    Transform::from_xyz(0.0, 0.0, 0.0),
                    Text2d::new(ball.n.to_string()),
                ));
            });
    }
}

fn render_balls(mut query: Query<(&BallComponent, &mut Transform)>, simulation: Res<Simulation>) {
    for (ball_visual, mut transform) in query.iter_mut() {
        let ball = &simulation.balls(ball_visual.billiard_id)[ball_visual.n];

        transform.translation.x = ball.position.x as f32;
        transform.translation.y = ball.position.y as f32;
        transform.rotation = Quat::from_axis_angle(
            Vec3::NEG_Z,
            f64::atan2(ball.velocity.x, ball.velocity.y) as f32,
        );
    }
}

fn handle_inputs(mut simulation: ResMut<Simulation>, keys: Res<ButtonInput<KeyCode>>) {
    if keys.just_pressed(KeyCode::Space) {
        simulation.toggle_pause();
    }
}
