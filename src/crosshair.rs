use bevy::app::{App, Plugin};
use bevy::color::Color;
use bevy::color::palettes::css::WHITE;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use crate::asset::SpaceKit;

pub struct CrossHairPlugin;

#[derive(Component)]
pub struct CrossHair;

impl Plugin for CrossHairPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_crosshair)
            .add_systems(Update, update_crosshair_position);
    }
}


fn spawn_crosshair(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>) {

    let material = materials.add(StandardMaterial {
        base_color: Color::srgba(1.0, 0.2, 0.2, 0.7),
        emissive: LinearRgba::from(Color::srgb(1.0, 0.3, 0.3)),
        alpha_mode: AlphaMode::Add,
        unlit: true,
        ..default()
    });
    commands.spawn((
        Mesh3d(meshes.add(Sphere::new(0.5).mesh().ico(5).unwrap())),
        MeshMaterial3d(material.clone()),
        Transform::from_xyz(0.0, 0.0, 17.0),
        CrossHair,
        ));
}
fn update_crosshair_position(
    windows: Query<&Window, With<PrimaryWindow>>,
    camera_query: Single<(&Camera, &GlobalTransform)>,
    mut crosshair_query: Query<&mut Transform, With<CrossHair>>,
    mut gizmos: Gizmos,
) {
    let (camera, camera_transform) = *camera_query;
    let window = windows.single();
    if let Some(cursor_pos) = window.cursor_position() {
        if let Ok(ray) = camera.viewport_to_world(camera_transform, cursor_pos) {
            let origin = ray.origin;
            let target = ray.origin + ray.direction * 100.0;

            // Draw a small crosshair at the point
            //gizmos.cross(Isometry3d::from_translation(target), 1.1, WHITE);
            let mut transform = crosshair_query.single_mut();
            transform.translation = target;

        }
    }
}