use bevy::math::Vec3;
use bevy::prelude::*;
use bevy_rapier3d::dynamics::{GravityScale, RigidBody};
use bevy_rapier3d::geometry::Collider;
use rand::{random, random_range};
use crate::asset::SpaceKit;

pub struct PlanetPlugin;

#[derive(Component, Debug)]
pub struct Planet;

impl Plugin for PlanetPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_planets);
    }
}

fn spawn_planets(
    mut commands: Commands,
    space_kit: Res<SpaceKit>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    for planet_handle in space_kit.planets.iter() {
        let scene_root = SceneRoot(planet_handle.clone());
        let x = random_range(-8000.0..8000.);
        let y = random_range(-8000.0..8000.);
        let z = random_range(-8000.0..8000.);
        let scale = 300.0;
        commands.spawn((
            scene_root,
            Transform {
                translation: Vec3::new(x, y, z),
                scale: Vec3::splat(scale),
                ..default()
            },
            RigidBody::Fixed,
            Mesh3d(meshes.add(Cuboid::default())),
            GravityScale(0.0),
            Collider::ball(scale / 150.),
        ));
    }
}