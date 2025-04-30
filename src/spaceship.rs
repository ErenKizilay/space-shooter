use crate::asset::SpaceKit;
use crate::bullet::spawn_bullet;
use crate::camera::MainCamera;
use bevy::app::{App, Plugin};
use bevy::prelude::*;
use bevy_rapier3d::geometry::Collider;
use bevy_rapier3d::math::Real;
use bevy_rapier3d::prelude::*;

use bevy::window::PrimaryWindow;
use rand::random_range;
use std::time::Duration;

pub struct SpaceshipPlugin;

#[derive(Component, Debug)]
pub struct SpaceShip;

#[derive(Component)]
struct LaserBeam;

#[derive(Component)]
struct ShipTrail;

#[derive(Event, Debug)]
pub struct SpaceshipThrusted {
    pub ship_transform: Transform,
}

#[derive(Resource, Debug)]
pub struct FireRate(Timer);
impl Plugin for SpaceshipPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(FireRate(Timer::new(Duration::from_millis(100), TimerMode::Repeating)))
            .add_systems(Startup, (spawn_space_ship, spawn_star_streaks))
            .add_systems(Update, (control_spaceship, fire_bullet))
            .add_event::<SpaceshipThrusted>();
    }
}

fn spawn_space_ship(
    mut commands: Commands,
    space_kit: Res<SpaceKit>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>
) {
    let exhaust_material = materials.add(StandardMaterial {
        base_color: Color::srgba(1.0, 0.2, 0.2, 0.7),
        emissive: LinearRgba::from(Color::srgb(1.0, 0.3, 0.3)),
        alpha_mode: AlphaMode::Add,
        unlit: true,
        ..default()
    });

    let scene_root = SceneRoot(space_kit.spaceship.clone());
    commands.spawn((
        scene_root,
        Transform::from_xyz(0.0, 0.0, 0.0)
            .looking_at(Vec3::from_array([0., 0., -1.]), Vec3::Y),
        ExternalForce::default(),
        Damping {
            linear_damping: 0.5,
            angular_damping: 1.0,
        },
        RigidBody::Dynamic,
        Collider::ball(2.),
        GravityScale(0.),
        Mesh3d(meshes.add(Capsule3d::default())),
        SpaceShip,
    )).with_children(|parent| {
        parent.spawn((
            Mesh3d(meshes.add(Sphere::new(0.5).mesh().ico(5).unwrap())),
            MeshMaterial3d(exhaust_material.clone()),
            Transform::from_xyz(1., 0.5, -5.0),
            Visibility::Hidden,
            ShipTrail,
            ));
        parent.spawn((
            Mesh3d(meshes.add(Sphere::new(0.5).mesh().ico(5).unwrap())),
            MeshMaterial3d(exhaust_material.clone()),
            Transform::from_xyz(-1., 0.5, -5.0),
            Visibility::Hidden,
            ShipTrail,
        ));
        parent.spawn((
            Mesh3d(meshes.add(Sphere::new(1.0).mesh().ico(5).unwrap())),
            MeshMaterial3d(exhaust_material.clone()),
            Transform::from_xyz(0.0, 0.0, 100.0),
        ));
    });
}

fn control_spaceship(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut ship_query: Query<(&Transform, &mut ExternalForce), With<SpaceShip>>,
    mut trail_query: Query<(&mut Visibility), With<ShipTrail>>,
    mut thrusted_events: EventWriter<SpaceshipThrusted>,
) {
    let (transform, mut force) = ship_query.single_mut();

    let forward = Vec3::from(transform.forward());
    let right = Vec3::from(transform.right());
    let up = Vec3::from(transform.up());

    let thrust_force = 600.0;
    let rotation_torque = 200.0;

    // Movement (W/S)
    let mut linear_force = Vec3::ZERO;
    let mut trail_visibility = Visibility::Hidden;
    if keyboard.pressed(KeyCode::KeyW) {
        linear_force -= forward;
        trail_visibility = Visibility::Visible;
        thrusted_events.send(SpaceshipThrusted {
            ship_transform: transform.clone(),
        });
    }
    if keyboard.pressed(KeyCode::KeyS) {
        linear_force += forward;
    }

    // Pitch (Up/Down)
    let mut angular_torque = Vec3::ZERO;
    if keyboard.pressed(KeyCode::ArrowUp) {
        angular_torque += right; // pitch up
    }
    if keyboard.pressed(KeyCode::ArrowDown) {
        angular_torque -= right; // pitch down
    }

    // Yaw (A/D)
    if keyboard.pressed(KeyCode::KeyA) {
        angular_torque += up; // yaw left
    }
    if keyboard.pressed(KeyCode::KeyD) {
        angular_torque -= up; // yaw right
    }

    // Roll (Q/E)
    if keyboard.pressed(KeyCode::KeyQ) {
        angular_torque += forward; // roll left
    }
    if keyboard.pressed(KeyCode::KeyE) {
        angular_torque -= forward; // roll right
    }

    for mut visibility in trail_query.iter_mut() {
        *visibility = trail_visibility;
    }

    force.force = linear_force.normalize_or_zero() * thrust_force;
    force.torque = angular_torque.normalize_or_zero() * rotation_torque;

    //println!("ship translation: {:?}", transform.translation);
}

fn fire_bullet(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    time: Res<Time>,
    mut fire_rate: ResMut<FireRate>,
    spaceship_query: Query<&Transform, (With<SpaceShip>, Without<MainCamera>)>,
    input: Res<ButtonInput<KeyCode>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    camera_q: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    if fire_rate.0.tick(time.delta()).just_finished() {
        if input.pressed(KeyCode::Space) {
            let spaceship_transform = spaceship_query.get_single().unwrap();
            // Forward direction in world space
            let forward = spaceship_transform.forward(); // Vec3
            // Bullet spawn position slightly in front of the ship
            let translation = spaceship_transform.translation;
            let spawn_position = translation + forward * -15.0;
            let spawn_position_x = translation + spaceship_transform.left() * 5.;
            let spawn_position_y = translation + spaceship_transform.right() * 5.;

            // Bullet speed
            let speed = 700.0;
            let forward = spaceship_transform.forward().normalize();
            for position in vec![spawn_position, spawn_position_x, spawn_position_y] {
                spawn_bullet(
                    &mut commands,
                    &mut meshes,
                    &mut materials,
                    Velocity {
                        linvel: -forward * speed,
                        ..default()
                    },
                    Transform::from_translation(position),
                );
            }
            /*if let Some(ray) = get_mouse_world_ray(windows, camera_q) {
                spawn_bullet(
                    &mut commands,
                    &mut meshes,
                    &mut materials,
                    Velocity {
                        linvel: ray.direction * speed,
                        ..default()
                    },
                    Transform::from_translation(spawn_position),
                );
            }*/
        }
    };
}

fn spawn_laser_beam(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let material = materials.add(StandardMaterial {
        base_color: Color::srgba(0.0, 1.0, 0.0, 0.5),
        emissive: LinearRgba::from(Color::srgb(0.0, 3.0, 0.0)),
        unlit: true,
        alpha_mode: AlphaMode::Add,
        ..default()
    });

    commands.spawn((
        Mesh3d(meshes.add(Cylinder::new(0.05, 1.0))),
        MeshMaterial3d(material),
        Transform::from_xyz(0.0, 0.0, 5.0),
        LaserBeam,
    ));
}

fn update_laser_beam(
    ship_query: Query<(Entity, &Transform), (With<SpaceShip>, Without<LaserBeam>)>,
    mut beam_query: Query<&mut Transform, With<LaserBeam>>,
    rapier_context: ReadRapierContext,
    mut gizmos: Gizmos,
) {
    let (ship_entity, ship_transform) = ship_query.single();
    let dir = -ship_transform.forward().as_vec3(); // ship looks along -Z
    let origin = ship_transform.translation;
    let max_distance: Real = 5000.0;

    let end = if let Some((_e, toi)) = rapier_context.single().cast_ray(
        origin,
        dir,
        max_distance,
        true,
        QueryFilter::default().exclude_collider(ship_entity),
    ) {
        println!("intersection: {:?}", _e);
        origin + dir * toi as f32
    } else {
        origin + dir * max_distance
    };

    let distance = origin.distance(end);
    let mut beam = beam_query.single_mut();

    let mid_point = (origin + end) / 2.0;
    beam.translation = origin + dir * 5.0;               // center the beam between start and end
    beam.look_to(end, Vec3::Y);   // rotate so Y axis points in the direction
    beam.scale = Vec3::new(0.5, 0.5, distance);

    //gizmos.ray(origin, end, BLUE);
}

fn spawn_laser(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let material = materials.add(StandardMaterial {
        base_color: Color::srgba(0.0, 1.0, 0.0, 0.5),
        emissive: LinearRgba::from(Color::srgb(0.0, 3.0, 0.0)),
        unlit: true,
        alpha_mode: AlphaMode::Add,
        ..default()
    });

    commands.spawn((
        Mesh3d(meshes.add(Cylinder::new(1., 1.0))),
        MeshMaterial3d(material),
        Transform {
            translation: Vec3::new(0.0, 0.3, 5.0),
            rotation: Quat::from_axis_angle(Vec3::Z, 0.0),
            scale: Vec3::new(1.0, 1.0, 500.0),
        }
    ));
}

#[derive(Component)]
struct StarStreak;
fn spawn_star_streaks(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let streak_material = materials.add(StandardMaterial {
        base_color: Color::srgba(1.0, 1.0, 1.0, 0.3),
        emissive: Color::WHITE.into(),
        alpha_mode: AlphaMode::Add,
        unlit: true,
        ..default()
    });

    let quad = meshes.add(Cuboid::new(0.2, 0.2, 0.2));

    for _ in 0..10000 {
        let x = random_range(-1000..1000) as f32;
        let y = random_range(-1000..1000) as f32;
        let z = random_range(-1000..1000) as f32;
        commands.spawn((
            Mesh3d(quad.clone()),
            MeshMaterial3d(streak_material.clone()),
            Transform {
                translation: Vec3::new(x, y, z),
                ..default()
            },
            StarStreak,
        ));
    }
}

fn get_mouse_world_ray(
    windows: Query<&Window, With<PrimaryWindow>>,
    camera_q: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) -> Option<Ray3d> {
    let (camera, cam_transform) = camera_q.single();
    let window = windows.single();

    let cursor_pos = window.cursor_position()?;
    let ray = camera.viewport_to_world(cam_transform, cursor_pos).unwrap();

    Some(ray)
}