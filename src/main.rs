use bevy::prelude::*;
use bevy_asset_loader::loading_state::{LoadingState, LoadingStateAppExt};
use bevy_ecs_tilemap::prelude::*;
use leafwing_input_manager::prelude::*;
use learning::{
    assets::{AssetsPlugin, ImageAssets, MyStates},
    player::{PlayerAction, controls, setup_player},
};

const WIDTH: u32 = 32;
const HEIGHT: u32 = 32;

fn startup(
    mut commands: Commands,
    image_assets: Res<ImageAssets>,
    array_texture_loader: Res<ArrayTextureLoader>,
) {
    commands.spawn(Camera2d);

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
        texture: TilemapTexture::Single(image_assets.terrain.clone()),
        tile_size,
        anchor: TilemapAnchor::Center,
        ..Default::default()
    });

    array_texture_loader.add(TilemapArrayTexture {
        texture: TilemapTexture::Single(image_assets.terrain.clone()),
        tile_size,
        ..Default::default()
    });
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<MyStates>()
        .add_plugins(InputManagerPlugin::<PlayerAction>::default())
        .add_plugins(AssetsPlugin)
        .add_loading_state(
            LoadingState::new(MyStates::AssetLoading).continue_to_state(MyStates::Next),
        )
        .add_plugins(TilemapPlugin)
        .add_systems(OnEnter(MyStates::Next), (startup, setup_player))
        .add_systems(Update, controls)
        .run();
}
