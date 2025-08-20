use bevy::{input::ButtonInput, prelude::*};
use bevy_ecs_tilemap::prelude::*;

mod map;

const WIDTH: u32 = 32;
const HEIGHT: u32 = 32;

fn startup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    array_texture_loader: Res<ArrayTextureLoader>,
) {
    commands.spawn(Camera2d);

    dbg!("Before first handle");
    let texture_handle: Handle<Image> = asset_server.load("terrain.png");
    dbg!("After first handle");

    let map_size = TilemapSize {
        x: WIDTH,
        y: HEIGHT,
    };

    let tilemap_entity = commands.spawn_empty().id();

    let mut tile_storage = TileStorage::empty(map_size);

    for x in 0..map_size.x {
        for y in 0..map_size.y {
            let tile_pos = TilePos { x, y };
            let tile_entity = commands
                .spawn(TileBundle {
                    position: tile_pos,
                    tilemap_id: TilemapId(tilemap_entity),
                    ..Default::default()
                })
                .id();

            tile_storage.set(&tile_pos, tile_entity);
        }
    }

    let tile_size = TilemapTileSize { x: 16.0, y: 16.0 };
    let grid_size = tile_size.into();
    let map_type = TilemapType::default();

    commands.entity(tilemap_entity).insert(TilemapBundle {
        grid_size,
        map_type,
        size: map_size,
        storage: tile_storage,
        texture: TilemapTexture::Single(texture_handle),
        tile_size,
        anchor: TilemapAnchor::Center,
        ..Default::default()
    });

    array_texture_loader.add(TilemapArrayTexture {
        texture: TilemapTexture::Single(asset_server.load("terrain.png")),
        tile_size,
        ..Default::default()
    });
}

#[derive(Component)]
struct Player;

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

fn setup_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let layout = texture_atlas_layouts.add(TextureAtlasLayout::from_grid(
        UVec2 { x: 16, y: 24 },
        3,
        1,
        None,
        None,
    ));

    commands.spawn((
        Sprite {
            image: asset_server.load("blob.png"),
            texture_atlas: Some(TextureAtlas { layout, index: 0 }),
            ..default()
        },
        Player,
        Transform::from_xyz(0.0, 0.0, 1.0),
        AnimationTimer(Timer::from_seconds(1., TimerMode::Repeating)),
    ));
}

fn controls(
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

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(TilemapPlugin)
        .add_systems(Startup, (startup, setup_player))
        .add_systems(Update, controls)
        .run();
}
