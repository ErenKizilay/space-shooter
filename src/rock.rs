use bevy::app::{App, Plugin, Startup, Update};
use bevy::asset::Assets;
use bevy::asset::io::memory::Value::Vec;
use bevy::math::Vec3;
use bevy_rapier3d::dynamics::{Damping, ExternalForce, RigidBody};
use crate::asset::SpaceKit;

use bevy::prelude::*;
use bevy_rapier3d::geometry::Collider;
use bevy_rapier3d::prelude::*;
use rand::Rng;
use crate::camera::MainCamera;
use crate::spaceship::SpaceShip;

pub struct RockPlugin;

#[derive(Component, Debug)]
pub struct Rock;
impl Plugin for RockPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (spawn_rocks))
            .add_systems(PostStartup, spawn_rocks)
            .add_systems(FixedUpdate, despawn_distant_rocks)
            .insert_resource(IntervalTimer(Timer::from_seconds(3.0, TimerMode::Repeating)));
    }
}

#[derive(Resource)]
struct IntervalTimer(Timer);

fn spawn_rocks(mut commands: Commands,
               space_kit: Res<SpaceKit>,
               mut meshes: ResMut<Assets<Mesh>>,
               spaceship_query: Query<&Transform, With<SpaceShip>>,
               time: Res<Time>,
               mut timer: ResMut<IntervalTimer>,) {
    if timer.0.tick(time.delta()).just_finished() {
        let spaceship_transform = spaceship_query.get_single().unwrap();
        let scene_root = SceneRoot(space_kit.rock.clone());
        commands.spawn(
            (scene_root, random_rock_transform(&spaceship_transform.translation),
             random_rock_velocity(),
             RigidBody::Dynamic,
             Collider::ball(1.),
             GravityScale(0.),
             Mesh3d(meshes.add(Cuboid::new(1., 1., 1.))), Rock));
    }
}

fn despawn_distant_rocks(mut commands: Commands,
          spaceship_query: Query<&Transform, With<SpaceShip>>,
          rock_query: Query<(&Transform, Entity), With<Rock>>) {
    let spaceship_transform = spaceship_query.get_single().unwrap();

    for (rock_transform, rock_entity)  in rock_query.iter() {
        let rock_translation = rock_transform.translation;
        let distance = rock_translation.distance(spaceship_transform.translation);
        if distance > 1500. {
            commands.entity(rock_entity).despawn();
        }
    }
}

fn random_rock_velocity() -> Velocity{
    let mut rng = rand::rng();
    let x = rng.random_range(-100.0..100.0);
    let y = rng.random_range(-100.0..100.0);
    let z = rng.random_range(-100.0..100.0);
    Velocity {
        linvel: Vec3::new(x, y, z),
        angvel: Default::default(),
    }
}

fn random_rock_transform(spaceship_translation: &Vec3) -> Transform {
    let mut rng = rand::rng();
    let x = rng.random_range(10.0..500.0);
    let y = rng.random_range(10.0..500.0);
    let z = rng.random_range(10.0..500.0);
    Transform {
        translation: Vec3::new(spaceship_translation.x + x, spaceship_translation.y + y, spaceship_translation.z + z),
        ..default()
    }
}