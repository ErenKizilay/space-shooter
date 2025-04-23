use bevy::prelude::*;

#[derive(Resource, Debug, Default)]
pub struct SpaceKit {
    pub spaceship: Handle<Scene>,
    pub enemy: Handle<Scene>,
    pub astronaut: Handle<Scene>,
    pub planets: Vec<Handle<Scene>>,
    pub rock: Handle<Scene>,
    pub mechs: Vec<Handle<Scene>>,
    pub skybox: Handle<Scene>,
}

pub struct AssetLoaderPlugin;

impl Plugin for AssetLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SpaceKit>()
            .add_systems(PreStartup, load_assets);
    }
}

fn load_assets(mut scene_assets: ResMut<SpaceKit>, asset_server: Res<AssetServer>) {
    *scene_assets = SpaceKit {
        spaceship: asset_server.load("Ultimate Space Kit-glb/Spaceship.glb#Scene0"),
        enemy: asset_server.load("Ultimate Space Kit-glb/Spaceship-Jqfed124pQ.glb#Scene0"),
        astronaut: asset_server.load("Ultimate Space Kit-glb/Astronaut.glb#Scene0"),
        planets: vec![
            asset_server.load("Ultimate Space Kit-glb/Planet.glb#Scene0"),
            asset_server.load("Ultimate Space Kit-glb/Planet-4NxxeyYMPJ.glb#Scene0"),
            asset_server.load("Ultimate Space Kit-glb/Planet-5zzi8WUMXj.glb#Scene0"),
            asset_server.load("Ultimate Space Kit-glb/Planet-9g1aIbfR9Y.glb#Scene0"),
            asset_server.load("Ultimate Space Kit-glb/Planet-18Uxrb2dIc.glb#Scene0"),
            asset_server.load("Ultimate Space Kit-glb/Planet-B7xd3SZq0z.glb#Scene0"),
            asset_server.load("Ultimate Space Kit-glb/Planet-EC1Lk2IamI.glb#Scene0"),
            asset_server.load("Ultimate Space Kit-glb/Planet-hKZtOOMadH.glb#Scene0"),
            asset_server.load("Ultimate Space Kit-glb/Planet-IVnmauIgWX.glb#Scene0"),
            asset_server.load("Ultimate Space Kit-glb/Planet-pHZz4EMvVM.glb#Scene0"),
            asset_server.load("Ultimate Space Kit-glb/Planet-rYguWNNPvA.glb#Scene0"),
        ],
        rock: asset_server.load("Ultimate Space Kit-glb/Rock.glb#Scene0"),
        mechs: vec![
            asset_server.load("Ultimate Space Kit-glb/Mech.glb#Scene0"),
            asset_server.load("Ultimate Space Kit-glb/Mech-4UvIHxnoSR.glb#Scene0"),
            asset_server.load("Ultimate Space Kit-glb/Mech-D5wW2jDO42.glb#Scene0"),
            asset_server.load("Ultimate Space Kit-glb/Mech-o3Ps8z8ByP.glb#Scene0"),
        ],
        skybox: asset_server.load("skybox/galaxy_panorama.glb#Scene0"),
    }
}