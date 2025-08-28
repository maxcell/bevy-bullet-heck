use bevy::{prelude::*, text::cosmic_text::ttf_parser::RgbaColor};

use crate::player::Player;

#[derive(Component)]
pub struct Weapon;

#[derive(Component)]
pub struct Laser;

#[derive(Resource)]
pub struct ShootingTimer(Timer);

const SHOOTING_INTERVAL: f32 = 1.0;

impl Default for ShootingTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(SHOOTING_INTERVAL, TimerMode::Repeating))
    }
}

pub fn player_shooting(
    mut commands: Commands,
    mut timer: ResMut<ShootingTimer>,
    time: Res<Time>,
    player: Single<(&Transform), With<Player>>,
) {
    if timer.0.finished() {
        commands.spawn((
            Laser,
            Sprite {
                color: Color::linear_rgb(255.0, 0.0, 0.0),
                custom_size: Some(Vec2 { x: 8.0, y: 8.0 }),
                ..Default::default()
            },
            Transform::from_translation(Vec3 {
                x: player.translation.x,
                y: player.translation.y,
                z: 1.0,
            })
            .with_rotation(player.rotation),
        ));
    }

    timer.0.tick(time.delta());
}

pub fn move_bullets(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform), With<Laser>>,
    time: Res<Time>,
) {
    let dt = time.delta_secs();

    dbg!(&query);

    for (entity, mut transform) in query.iter_mut() {
        let forward_x = transform.local_x();
        let forward_y = transform.local_y();

        transform.translation += 32.0 * (forward_x) * dt;
    }
}
