use crate::mech::Mech;
use bevy::app::{App, Plugin};
use bevy::asset::{AssetContainer, Assets};
use bevy_rapier3d::dynamics::{GravityScale, RigidBody};
use std::collections::HashSet;
use std::time::Duration;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_rapier3d::prelude::*;
use crate::camera::MainCamera;

#[derive(Component, Debug)]
pub struct Bullet(Timer);

#[derive(Event, Debug)]
pub struct BulletHit {
    pub count: u32
}

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (detect_collision, despawn_bullet))
            .add_event::<BulletHit>();
    }
}

pub fn spawn_bullet(
    mut commands: &mut Commands,
    mut meshes: &mut ResMut<Assets<Mesh>>,
    mut materials: &mut ResMut<Assets<StandardMaterial>>,
    velocity: Velocity,
    transform: Transform,
) {
    let material = materials.add(StandardMaterial {
        base_color: Color::srgba(1.0, 0.0, 0.0, 0.9), // Semi-transparent red
        emissive: LinearRgba::from(Color::srgb(10.0, 0.1, 0.1)),        // Intense red glow
        alpha_mode: AlphaMode::Add,                 // Additive blending
        unlit: true,
        ..default()
    });
    commands.spawn((
        transform,
        velocity,
        RigidBody::Dynamic,
        Mesh3d(meshes.add(Capsule3d {
            radius: 0.3,
            half_length: 0.05,
        })),
        Collider::capsule_z(1.0, 1.0),
        MeshMaterial3d(material),
        GravityScale(0.0),
        Bullet(Timer::new(Duration::from_secs(5), TimerMode::Once)),
    )).insert(ActiveEvents::COLLISION_EVENTS);
}

fn detect_collision(
    mut collision_events: EventReader<CollisionEvent>,
    mut bullet_hit_events: EventWriter<BulletHit>,
    bullet_query: Query<Entity, With<Bullet>>,
    target_query: Query<Entity, With<Mech>>,
) {
    let bullets: HashSet<Entity> = bullet_query.iter().collect();
    let targets: HashSet<Entity> = target_query.iter().collect();
    let mut hit_count: u32 = 0;
    collision_events.read().for_each(|event| match event {
        CollisionEvent::Started(e1, e2, flags) => {
            let bullet_involved = bullets.contains(e1) || bullets.contains(e2);
            let mech_involved = targets.contains(e1) || targets.contains(e2);
            if bullet_involved && mech_involved {
                println!("Bullet is involved the collision.");
                hit_count += 1;
            }

        },
        CollisionEvent::Stopped(_, _, _) => {}
    });
    bullet_hit_events.send(BulletHit { count: hit_count });
}

fn despawn_bullet(
    mut commands: Commands,
    time: Res<Time>,
    mut bullet_query: Query<(Entity, &mut Bullet), With<Bullet>>,) {
    bullet_query.iter_mut().for_each(|(entity,mut bullet)| {
        let mut timer = &mut bullet.0;
        if timer.tick(time.delta()).just_finished() {
            commands.entity(entity).despawn_recursive();
            //println!("Despawned bullet entity.");
        };
    })
}
