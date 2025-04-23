//! Illustrates different lights of various types and colors, some static, some moving over
//! a simple scene.

mod asset;
mod bullet;
mod camera;
mod game;
mod mech;
mod planet;
mod rock;
mod spaceship;
mod enemy;

use crate::asset::AssetLoaderPlugin;
use crate::bullet::BulletPlugin;
use crate::camera::CameraPlugin;
use crate::enemy::EnemyPlugin;
use crate::game::GamePlugin;
use crate::mech::MecPlugin;
use crate::planet::PlanetPlugin;
use crate::rock::RockPlugin;
use crate::spaceship::SpaceshipPlugin;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        //.add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(CameraPlugin)
        .add_plugins(AssetLoaderPlugin)
        .add_plugins(SpaceshipPlugin)
        .add_plugins(RockPlugin)
        .add_plugins(MecPlugin)
        .add_plugins(BulletPlugin)
        .add_plugins(PlanetPlugin)
        .add_plugins(GamePlugin)
        .add_plugins(EnemyPlugin)
        .run();
}
