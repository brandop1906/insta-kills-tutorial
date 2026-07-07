use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use crate::animation::AnimationClip2D;

#[derive(AssetCollection, Resource)]
pub struct GameAssets {
    // The sprite sheet: one image, plus a layout describing the grid of frames.
    #[asset(texture_atlas_layout(tile_size_x = 80, tile_size_y = 80, columns = 23, rows = 5))]
    pub player_layout: Handle<TextureAtlasLayout>,
    #[asset(path = "player/player.png")]
    pub player: Handle<Image>,

    #[asset(path = "player/player_shadow.png")]
    pub player_shadow: Handle<Image>,

    #[asset(path = "fonts/PressStart2P.ttf")]
    pub font: Handle<Font>,

    #[asset(paths(
        "player/player.trickfilm#idle",
        "player/player.trickfilm#moving",
        "player/player.trickfilm#dashing",
        "player/player.trickfilm#hooking",
        "player/player.trickfilm#sliding",
    ), collection(typed))]
    pub player_animations: Vec<Handle<AnimationClip2D>>,

}
