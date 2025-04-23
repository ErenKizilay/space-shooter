use std::time::Duration;
use bevy::app::App;
use bevy::asset::Assets;
use bevy::math::Vec3;
use bevy::prelude::*;
use bevy_rapier3d::dynamics::{Damping, ExternalForce, GravityScale, RigidBody, Velocity};
use bevy_rapier3d::geometry::Collider;
use rand::random_range;
use crate::asset::SpaceKit;
use crate::bullet::{spawn_bullet, Bullet};
use crate::mech::{Mech, RandomFlight};
use crate::spaceship::SpaceShip;

#[derive(Component, Debug)]
pub struct Enemy;

#[derive(Resource, Debug)]
pub struct EnemyFireRate(Timer);

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(EnemyFireRate(Timer::new(Duration::from_secs(1), TimerMode::Repeating)))
            .add_systems(PostStartup, spawn_enemy)
            .add_systems(Update, attack);
    }
}

fn spawn_enemy(
    mut commands: Commands,
    space_kit: Res<SpaceKit>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let scene_root = SceneRoot(space_kit.enemy.clone());
    for i in 1..100 {
        let x = random_range(-1000..1000) as f32;
        let y = random_range(-1000..1000) as f32;
        let z = random_range(-1000..1000) as f32;
        commands.spawn((
            scene_root.clone(),
            Transform::from_xyz(x, y, z),
            ExternalForce::default(),
            Damping {
                linear_damping: 0.5,
                angular_damping: 1.0,
            },
            RigidBody::Dynamic,
            Collider::ball(2.),
            GravityScale(0.),
            Mesh3d(meshes.add(Capsule3d::default())),
            Enemy,
            RandomFlight {
                direction: Vec3::ZERO,
                timer: Timer::new(Duration::from_secs(2), TimerMode::Repeating)
            },
        ));
    }
}

fn attack(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    time: Res<Time>,
    mut enemy_fire_rate: ResMut<EnemyFireRate>,
    spaceship_query: Query<&Transform, With<SpaceShip>>,
    enemy_query: Query<(Entity, &Transform), With<Enemy>>,
) {
    if enemy_fire_rate.0.tick(time.delta()).just_finished() {
        if let Ok(spaceship_transform) = spaceship_query.get_single() {
            for (enemy_entity, enemy_transform) in enemy_query.iter() {
                if spaceship_transform.translation.distance(enemy_transform.translation) < 500.0 {
                    // Update enemy rotation to face spaceship
                    let new_rotation = enemy_transform
                        .looking_at(spaceship_transform.translation, Vec3::Y)
                        .rotation;

                    commands.entity(enemy_entity).insert(Transform {
                        rotation: new_rotation,
                        ..*enemy_transform
                    });

                    let spawn_position = enemy_transform.translation
                        + (spaceship_transform.translation - enemy_transform.translation)
                        .normalize()
                        * 15.0;

                    spawn_bullet(
                        &mut commands,
                        &mut meshes,
                        &mut materials,
                        Velocity {
                            linvel: (spaceship_transform.translation - enemy_transform.translation)
                                .normalize()
                                * 100.0,
                            ..default()
                        },
                        Transform::from_translation(spawn_position),
                    );

                    println!("Enemy fired!");
                }
            }
        }
    };
}