use crate::spaceship::SpaceShip;
use bevy::pbr::{CascadeShadowConfigBuilder, NotShadowCaster};
use bevy::prelude::*;
use std::f32::consts::PI;
use bevy_rapier3d::dynamics::{GravityScale, RigidBody};
use crate::asset::SpaceKit;
use bevy::render::camera::Viewport;

const CAMERA_DISTANCE: f32 = 80.0;

#[derive(Component, Debug)]
pub struct MainCamera;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_camera, spawn_skybox));
        app.add_systems(Update, follow_spaceship_smooth);
    }
}

fn spawn_camera(mut commands: Commands) {
    // Add a camera so we can see the debug-render.
    // directional 'sun' light
    commands.spawn((
        DirectionalLight {
            illuminance: light_consts::lux::AMBIENT_DAYLIGHT,
            shadows_enabled: true,
            ..default()
        },
        Transform {
            translation: Vec3::new(0.0, 10.0, 15.),
            rotation: Quat::from_rotation_x(-PI / 4.),
            ..default()
        },
        // The default cascade config is designed to handle large scenes.
        // As this example has a much smaller world, we can tighten the shadow
        // bounds for better visual quality.
        CascadeShadowConfigBuilder {
            first_cascade_far_bound: 4.0,
            maximum_distance: 10.0,
            ..default()
        }
        .build(),
    ));
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0., 10.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        MainCamera,
    ));


}

fn follow_spaceship_smooth(
    spaceship_query: Query<&Transform, (With<SpaceShip>, Without<MainCamera>)>,
    mut camera_query: Query<&mut Transform, With<MainCamera>>,
    time: Res<Time>,
) {
    let spaceship_transform = spaceship_query.single();

    // Offset behind and slightly above the spaceship in its local space
    let local_offset = spaceship_transform.forward() * 20.0 + Vec3::Y * 10.0;
    let target_position = spaceship_transform.translation + local_offset;

    if let Ok(mut camera_transform) = camera_query.get_single_mut() {
        // Smooth interpolation
        camera_transform.translation = camera_transform
            .translation
            .lerp(target_position, 5.0 * time.delta_secs());

        // Make the camera look at the spaceship
        camera_transform.look_at(spaceship_transform.translation, Vec3::Y);
    }
}

fn spawn_skybox(mut commands: Commands, space_kit: Res<SpaceKit>,) {
    let scene_root = SceneRoot(space_kit.skybox.clone());
    commands.spawn(
        (scene_root, Transform {
            translation: Vec3::ZERO,
            scale: Vec3::splat(10000.0), // or larger depending on model
            ..default()
        },
         NotShadowCaster
        ));
}
