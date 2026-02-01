use crate::components::*;
use crate::constants::*;
use bevy::prelude::Handle;
use bevy::prelude::*;
use bevy_ecs_tiled::prelude::*;

use crate::post_processing;

pub fn setup_scene(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    // Spawn Camera
    commands.spawn((
        Camera2d, // Add the setting to the camera.
        // This component is also used to determine on which camera to run the post processing effect.
        post_processing::PostProcessSettings {
            intensity: 25.2,
            ..default()
        },
    ));

    // Spawn tilemap
    // Load a map asset and retrieve its handle
    let map_handle: Handle<TiledMapAsset> = asset_server.load("Trees.tmx");

    // Spawn a new entity with the TiledMap component
    let background_tilemap = commands.spawn(TiledMap(map_handle)).id();
    commands
        .entity(background_tilemap)
        .insert(Transform::from_xyz(-328., -232., LAYER_TILEMAP as f32));

    // Spawn Application State, this controls the state of the application
    commands.spawn((RoundColorState::default(), RoundColorState2));

    // Spawn Application State, this controls the state of the application
    commands.spawn(GlobalApplicationState::default());

    // Spawn Turn State, this controls the turn based mechanics of the game.
    commands.spawn(GlobalTurnState::default());

    let texture = asset_server.load("Player.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(16), 3, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    // Use only the subset of sprites in the sheet that make up the run animation
    let animation_indices = AnimationIndices { first: 0, last: 2 };

    // Spawn Player: troop, player, sprite and transform components.
    commands.spawn((
        Player,
        TilePos { x: 8, y: 3 },
        Transform::from_xyz(0., 0., LAYER_PLAYER as f32),
        Sprite::from_atlas_image(
            texture,
            TextureAtlas {
                layout: texture_atlas_layout,
                index: animation_indices.first,
            },
        ),
        animation_indices,
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
    ));

    // Playable zone tilemap
    let texture_handle: Handle<Image> = asset_server.load("Playzone-Tilemap.png");

    // Size of the tile map in tiles.
    let map_size = TilemapSize {
        x: TILEMAP_SIDE_LENGHT,
        y: TILEMAP_SIDE_LENGHT,
    };

    // To create a map we use the TileStorage component.
    // This component is a grid of tile entities and is used to help keep track of individual
    // tiles in the world.
    let mut tile_storage = TileStorage::empty(map_size);

    let map_type = TilemapType::Square;

    // We need to tell each tile which tilemap entity
    // it is associated with. This is done with the TilemapId component on each tile.
    let tilemap_entity = commands.spawn_empty().id();

    // Spawn the tilemap.
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

            // Here I set the tile storage to component know what tiles we have like the graphical content.
            tile_storage.set(&tile_pos, tile_entity);
            if let Some(tile_entity) = tile_storage.get(&tile_pos) {
                commands.entity(tile_entity).insert(TileTextureIndex(0));
            }
        }
    }

    // This is the size of each individual tiles in pixels.
    let tile_size = TilemapTileSize {
        x: TILEMAP_SIDE_LENGHT as f32,
        y: TILEMAP_SIDE_LENGHT as f32,
    };
    let grid_size = tile_size.into();

    // Once the tile storage is inserted onto the tilemap entity it can no longer be accessed.
    commands.entity(tilemap_entity).insert((
        TilemapBundle {
            grid_size,
            size: map_size,
            storage: tile_storage,
            map_type,
            texture: TilemapTexture::Single(texture_handle),
            tile_size,
            anchor: TilemapAnchor::Center,
            ..Default::default()
        },
        PlayZoneTilemap,
    ));

    commands
        .entity(tilemap_entity)
        .insert(Transform::from_xyz(0., 0., LAYER_TILEMAP as f32));
}
