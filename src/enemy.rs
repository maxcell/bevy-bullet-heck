use bevy::prelude::*;
use rand::prelude::*;

use crate::{assets::ImageAssets, player::Player};

#[derive(Component)]
pub struct Enemy;

pub fn spawn_enemy(
    mut commands: Commands,
    image_assets: Res<ImageAssets>,
    player_transform: Query<&Transform, With<Player>>,
) {
    if player_transform.is_empty() {
        return;
    }

    let player_x = player_transform
        .single()
        .expect("Should be one")
        .clone()
        .translation
        .x;

    let player_y = player_transform
        .single()
        .expect("Should be one")
        .clone()
        .translation
        .y;

    let mut rand = rand::rng();

    let x_distance = rand.random_range(-200.0..=200.0);
    let y_distance = rand.random_range(-200.0..=200.0);

    let layout = image_assets.player_layout.clone();
    commands.spawn((
        Enemy,
        Sprite {
            image: image_assets.player.clone(),
            texture_atlas: Some(TextureAtlas { layout, index: 1 }),
            ..default()
        },
        Transform::from_xyz(player_x + x_distance, player_y + y_distance, 1.0),
    ));
}

pub fn update_enemy_position(
    player_transform: Query<&Transform, With<Player>>,
    mut enemy_transform: Query<&mut Transform, (With<Enemy>, Without<Player>)>,
) {
    if player_transform.is_empty() || enemy_transform.is_empty() {
        return;
    }

    let player_pos = player_transform
        .single()
        .expect("should be one")
        .translation;

    for mut transform in enemy_transform.iter_mut() {
        let dir = (player_pos - transform.translation).normalize();

        transform.translation += dir * 1.0;
    }
}
