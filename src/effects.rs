use std::time::Duration;
use bevy::app::App;
use bevy::prelude::*;
use rand::random_range;
use crate::camera::MainCamera;
use crate::spaceship::SpaceshipThrusted;

#[derive(Component)]
pub struct SpaceshipTrail {
    timer: Timer
}

pub struct EffectsPlugin;

impl Plugin for EffectsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (spawn_trail, update_trails));
    }
}

pub fn spawn_trail(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    camera_query: Single<(&Transform), With<MainCamera>>,
    mut event_reader: EventReader<SpaceshipThrusted>) {

    let material = materials.add(StandardMaterial {
        base_color: Color::srgba(1.0, 0.5, 0.0, 0.6),
        emissive: LinearRgba::from(Color::srgb(4.0, 2.0, 0.0)),
        alpha_mode: AlphaMode::Add, // For glowing blend
        cull_mode: None,
        ..default()
    });
    event_reader.read().for_each(|event| {
        for _ in 1..3 {
            let ship_transform = event.ship_transform;
            let forward = ship_transform.forward(); // -Z in Bevy by default
            let right = ship_transform.right();     // +X
            let up = ship_transform.up();           // +Y

            // Randomize offset in local space
            let offset = right * random_range(-0.2..0.2)
                + up * random_range(-0.2..0.2)
                - forward * random_range(-7.0..-5.0); // behind the ship

            let back_position = ship_transform.translation + offset;

            commands.spawn((
                Mesh3d(meshes.add(Mesh::from(Rectangle::new(0.2, 0.2)))),
                MeshMaterial3d(material.clone()),
                Transform {
                    translation: back_position,
                    scale: Vec3::splat(random_range(0.2..0.5) as f32),
                    ..default()
                }.looking_at(camera_query.translation, Vec3::Z),
                SpaceshipTrail {
                    timer: Timer::new(Duration::from_secs(1), TimerMode::Once),
                },
            ));
        }
    });
}

fn update_trails(
    mut commands: Commands,
    mut query: Query<(Entity,&mut SpaceshipTrail)>,
    time: Res<Time>) {
    for (entity, mut trail) in query.iter_mut() {
        trail.timer.tick(time.delta());
        if trail.timer.just_finished() {
            commands.entity(entity).despawn();
        }
    }
}