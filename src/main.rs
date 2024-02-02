use bevy::{
    input::common_conditions::input_just_pressed,
    math::vec2,
    prelude::*,
    reflect::TypeUuid,
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::{Material2d, Material2dPlugin, MaterialMesh2dBundle},
    window::{PrimaryWindow, WindowResolution},
};
use bevy_shader_utils::ShaderUtilsPlugin;
use lazy_static::lazy_static;
use std::sync::{Arc, Mutex};

const SIZE: (f32, f32) = (1000.0, 1000.0);

lazy_static! {
    static ref UL: Arc<Mutex<Vec2>> = Arc::new(Mutex::new(vec2(-0.75, 0.75)));
    static ref LR: Arc<Mutex<Vec2>> = Arc::new(Mutex::new(vec2(0., 0.)));
}

fn lock_ul() -> std::sync::MutexGuard<'static, Vec2> {
    UL.lock().expect("Failed to lock UL")
}

fn lock_lr() -> std::sync::MutexGuard<'static, Vec2> {
    LR.lock().expect("Failed to lock LR")
}

fn main() {
    let window_plugin = WindowPlugin {
        primary_window: Some(Window {
            resolution: WindowResolution::new(SIZE.0, SIZE.1),
            resizable: false,
            ..default()
        }),
        ..default()
    };

    App::new()
        .add_plugins((
            DefaultPlugins.set(window_plugin),
            ShaderUtilsPlugin,
            Material2dPlugin::<CompexPlaneMaterial>::default(),
        ))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            mouse_navigation.run_if(input_just_pressed(MouseButton::Left)),
        )
        .add_systems(
            Update,
            keyboard_navigation_zoom_in.run_if(input_just_pressed(KeyCode::Up)),
        )
        .add_systems(
            Update,
            keyboard_navigation_zoom_out.run_if(input_just_pressed(KeyCode::Down)),
        )
        .run();
}

fn recalculate_coordinates(px: f32, py: f32, zoom: f32) {
    let ul: &mut Vec2 = &mut lock_ul();
    let lr: &mut Vec2 = &mut lock_lr();

    let c = (*ul + *lr) / 2.;
    let m = vec2((1. - px) * ul.x + px * lr.x, (1. - py) * ul.y + py * lr.y);

    *ul += m - c;
    *lr += m - c;

    let padding = (c - *ul) / 8. * zoom;
    *ul += padding;
    *lr -= padding;
}

fn redraw(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut complex_plane_materials: ResMut<Assets<CompexPlaneMaterial>>,
) {
    let ul;
    let lr;
    {
        ul = lock_ul().clone();
        lr = lock_lr().clone();
    }

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
            transform: Transform::default().with_scale(Vec3::splat(SIZE.0.min(SIZE.1))),
            material: complex_plane_materials.add(CompexPlaneMaterial {
                ul_re: ul.x,
                ul_im: ul.y,
                lr_re: lr.x,
                lr_im: lr.y,
            }),
            visibility: Visibility::Visible,
            ..default()
        },
        Image,
    ));
}

fn mouse_navigation(
    mut commands: Commands,
    windows: Query<&Window, With<PrimaryWindow>>,
    previous_image: Query<Entity, With<Image>>,
    meshes: ResMut<Assets<Mesh>>,
    complex_plane_materials: ResMut<Assets<CompexPlaneMaterial>>,
) {
    let position = windows
        .single()
        .cursor_position()
        .expect("Didn't find cursor")
        / vec2(SIZE.0, SIZE.1);

    recalculate_coordinates(position.x, position.y, 1.0);

    let previous = previous_image.single();
    commands.entity(previous).despawn_recursive();

    redraw(commands, meshes, complex_plane_materials)
}

fn keyboard_navigation_zoom_in(
    mut commands: Commands,
    previous_image: Query<Entity, With<Image>>,
    meshes: ResMut<Assets<Mesh>>,
    complex_plane_materials: ResMut<Assets<CompexPlaneMaterial>>,
) {
    recalculate_coordinates(0.5, 0.5, 1.0);

    let previous = previous_image.single();
    commands.entity(previous).despawn_recursive();

    redraw(commands, meshes, complex_plane_materials)
}

fn keyboard_navigation_zoom_out(
    mut commands: Commands,
    previous_image: Query<Entity, With<Image>>,
    meshes: ResMut<Assets<Mesh>>,
    complex_plane_materials: ResMut<Assets<CompexPlaneMaterial>>,
) {
    recalculate_coordinates(0.5, 0.5, -1.0);

    let previous = previous_image.single();
    commands.entity(previous).despawn_recursive();

    redraw(commands, meshes, complex_plane_materials)
}

#[derive(Component)]
struct Image;

fn setup(
    mut commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    complex_plane_materials: ResMut<Assets<CompexPlaneMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());

    redraw(commands, meshes, complex_plane_materials)
}

// Set up materials
#[derive(Asset, AsBindGroup, TypeUuid, Debug, Clone, Reflect)]
#[uuid = "08063870-7da9-4b79-b9b7-6eeb904222ed"]
pub struct CompexPlaneMaterial {
    #[uniform(0)]
    ul_re: f32,
    #[uniform(0)]
    ul_im: f32,
    #[uniform(0)]
    lr_re: f32,
    #[uniform(0)]
    lr_im: f32,
}

impl Material2d for CompexPlaneMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/mandelbrot.wgsl".into()
    }
}
