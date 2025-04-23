use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use crate::bullet::BulletHit;

#[derive(Resource)]
pub struct GameState {
    pub score: u32,
}

#[derive(Component)]
pub struct ScoreText;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_ui)
            .insert_resource(GameState { score: 0 })
            .add_systems(Update, update_score);
    }
}

fn update_score(mut game_state: ResMut<GameState>,
                mut bullet_hit_events: EventReader<BulletHit>,
                mut score_text: Query<&mut Text, With<ScoreText>>) {
    bullet_hit_events.read().for_each(&mut |bullet_hit: &BulletHit| {
        game_state.score += bullet_hit.count;
    });
    if let Ok(mut text) = score_text.get_single_mut() {
        text.0 = format!("Score: {}", game_state.score);
    }
}

fn setup_ui(mut commands: Commands) {
    commands.spawn((
        Text("Score: 0".to_string()),
        TextLayout::new_with_justify(JustifyText::Left),
        ScoreText,
    ));
}
