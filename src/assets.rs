use bevy::{
    app::{App, Plugin},
    asset::Handle,
    ecs::resource::Resource,
    image::{Image, TextureAtlasLayout},
    state::state::States,
};
use bevy_asset_loader::prelude::*;

#[derive(AssetCollection, Resource)]
pub struct ImageAssets {
    #[asset(path = "terrain.png")]
    pub terrain: Handle<Image>,
    #[asset(texture_atlas_layout(
        tile_size_x = 16,
        tile_size_y = 24,
        columns = 3,
        rows = 1,
        padding_x = 0,
        padding_y = 0,
    ))]
    pub player_layout: Handle<TextureAtlasLayout>,
    #[asset(path = "blob.png")]
    pub player: Handle<Image>,
}

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
pub enum MyStates {
    #[default]
    AssetLoading,
    Next,
}

pub struct AssetsPlugin;

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app.init_collection::<ImageAssets>();
    }
}
