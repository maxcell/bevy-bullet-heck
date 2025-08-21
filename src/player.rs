use bevy::prelude::*;

use crate::assets::ImageAssets;

#[derive(Component)]
pub struct Player;

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(Timer);

pub fn setup_player(mut commands: Commands, image_assets: Res<ImageAssets>) {
    let layout = image_assets.player_layout.clone();

    commands.spawn((
        Sprite {
            image: image_assets.player.clone(),
            texture_atlas: Some(TextureAtlas { layout, index: 0 }),
            ..default()
        },
        Player,
        Transform::from_xyz(0.0, 0.0, 1.0),
        AnimationTimer(Timer::from_seconds(1., TimerMode::Repeating)),
    ));
}

pub fn controls(
    player_sprite: Single<(&mut Transform, &mut AnimationTimer, &mut Sprite), With<Player>>,
    buttons: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let (mut sprite_position, mut timer, mut sprite) = player_sprite.into_inner();

    let distance = 16. * time.delta_secs() * 2.;
    if buttons.pressed(KeyCode::ArrowRight) {
        dbg!("RIGHT ARROW PRESSED");
        sprite_position.translation.x += distance
    }

    if buttons.pressed(KeyCode::ArrowLeft) {
        dbg!("LEFT ARROW PRESSED");
        sprite_position.translation.x -= distance
    }

    if buttons.pressed(KeyCode::ArrowUp) {
        dbg!("UP ARROW PRESSED");
        sprite_position.translation.y += distance
    }

    if buttons.pressed(KeyCode::ArrowDown) {
        dbg!("DOWN ARROW PRESSED");
        sprite_position.translation.y -= distance
    }

    timer.tick(time.delta());

    if timer.just_finished() {
        if let Some(atlas) = &mut sprite.texture_atlas {
            atlas.index = if atlas.index == 1 { 0 } else { 1 };
        }
    }
}
