use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::assets::ImageAssets;

#[derive(Component)]
pub struct Player;

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(Timer);

#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
pub enum PlayerAction {
    #[actionlike(DualAxis)]
    Move,
    UseItem,
}

impl PlayerAction {
    /// Define the default bindings to the input
    fn default_input_map() -> InputMap<Self> {
        let mut input_map = InputMap::default();

        // Default gamepad input bindings
        input_map.insert_dual_axis(Self::Move, GamepadStick::LEFT);
        input_map.insert(Self::UseItem, GamepadButton::RightTrigger2);

        // Default kbm input bindings
        input_map.insert_dual_axis(Self::Move, VirtualDPad::wasd());
        input_map.insert_dual_axis(Self::Move, VirtualDPad::arrow_keys());
        input_map.insert(Self::UseItem, MouseButton::Left);

        input_map
    }
}

pub fn setup_player(mut commands: Commands, image_assets: Res<ImageAssets>) {
    let layout = image_assets.player_layout.clone();

    commands.spawn(PlayerAction::default_input_map()).insert((
        Sprite {
            image: image_assets.player.clone(),
            texture_atlas: Some(TextureAtlas { layout, index: 0 }),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 1.0),
        AnimationTimer(Timer::from_seconds(1., TimerMode::Repeating)),
        Player,
    ));
}

pub fn controls(
    player_sprite: Single<(&mut Transform, &mut AnimationTimer, &mut Sprite), With<Player>>,
    query: Query<&ActionState<PlayerAction>, With<Player>>,
    time: Res<Time>,
) {
    let (mut sprite_position, mut timer, mut sprite) = player_sprite.into_inner();

    let action_state = query.single().expect("Player actions not found");
    let axis_pair = action_state.clamped_axis_pair(&PlayerAction::Move);

    let distance = 16. * time.delta_secs() * 2.;
    if axis_pair.x > 0. {
        dbg!("Directionally right");
        sprite_position.translation.x += distance
    }

    if axis_pair.x < 0. {
        dbg!("Directionally left");
        sprite_position.translation.x -= distance
    }

    if axis_pair.y > 0. {
        dbg!("Directionally up");
        sprite_position.translation.y += distance
    }

    if axis_pair.y < 0. {
        dbg!("Directionally down");
        sprite_position.translation.y -= distance
    }

    timer.tick(time.delta());

    if timer.just_finished() {
        if let Some(atlas) = &mut sprite.texture_atlas {
            atlas.index = if atlas.index == 1 { 0 } else { 1 };
        }
    }
}
