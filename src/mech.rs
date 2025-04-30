use crate::asset::SpaceKit;
use bevy::app::{App, Plugin, Startup, Update};
use bevy::asset::Assets;
use bevy::math::Vec3;
use bevy::prelude::*;
use bevy_rapier3d::dynamics::{Damping, ExternalForce, GravityScale, RigidBody};
use bevy_rapier3d::geometry::Collider;
use rand::{random, random_range};

pub struct MecPlugin;

#[derive(Component)]
pub struct RandomFlight {
    pub(crate) direction: Vec3,
    pub(crate) timer: Timer,
}
#[derive(Component, Debug)]
pub struct Mech;
impl Plugin for MecPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_mech))
            .add_systems(Update, random_flight_system);
    }
}

fn spawn_mech(
    mut commands: Commands,
    space_kit: Res<SpaceKit>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    for i in 1..1000 {
        let index = random_range(0..space_kit.mechs.len());
        let scene_root = SceneRoot(space_kit.mechs.get(index).unwrap().clone());
        let x: f32 = random_range(-2000..2000) as f32;
        let y: f32 = random_range(-2000..2000) as f32;
        let z: f32 = random_range(-2000..2000) as f32;
        commands.spawn((
            scene_root,
            Transform::from_xyz(x, y, z)
                .looking_at(Vec3::from_array([0., 0., 0.]), Vec3::Y),
            ExternalForce::default(),
            Damping {
                linear_damping: 0.5,
                angular_damping: 1.0,
            },
            RigidBody::Dynamic,
            Collider::ball(2.),
            GravityScale(0.),
            Mesh3d(meshes.add(Capsule3d::default())),
            Mech,
            RandomFlight {
                direction: Vec3::new(
                    random::<f32>() * 2.0 - 1.0,
                    random::<f32>() * 2.0 - 1.0,
                    random::<f32>() * 2.0 - 1.0,
                ).normalize(),
                timer: Timer::from_seconds(3.0, TimerMode::Repeating),
            }
        ));
    }
}

fn random_flight_system(
    time: Res<Time>,
    mut query: Query<(&mut ExternalForce, &mut RandomFlight)>,
) {
    for (mut force, mut flight) in query.iter_mut() {
        // Update timer
        flight.timer.tick(time.delta());

        // If it's time, change direction randomly
        if flight.timer.finished() {
            flight.direction = Vec3::new(
                random::<f32>() * 2.0 - 1.0,
                random::<f32>() * 2.0 - 1.0,
                random::<f32>() * 2.0 - 1.0,
            )
                .normalize();
        }

        // Apply force in current direction
        force.force = flight.direction * 200.0; // Tune magnitude
    }
}