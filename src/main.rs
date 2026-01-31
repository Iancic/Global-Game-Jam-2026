use bevy::{input_focus::InputFocus, prelude::*};
use bevy_ecs_tiled::prelude::*;
use bevy_egui::{EguiPlugin, EguiPrimaryContextPass};
use bevy::window::WindowMode;
use bevy::prelude::Handle;

pub mod camera;
pub mod editor;
pub mod helpers;
pub mod button;

pub mod constants;
pub mod troop_utilities;
pub mod components;
use crate::constants::*;
use crate::troop_utilities::*;
use crate::components::*;

fn startup(mut commands: Commands, asset_server: Res<AssetServer>, mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>) {
    // Spawn Camera
    commands.spawn(Camera2d);

    // Spawn tilemap
    // Load a map asset and retrieve its handle
    let map_handle: Handle<TiledMapAsset> = asset_server.load("Trees.tmx");

    // Spawn a new entity with the TiledMap component
    let background_tilemap = commands.spawn(TiledMap(map_handle)).id();
    commands.entity(background_tilemap).insert(Transform::from_xyz(-328., -232., LAYER_TILEMAP as f32));

    // Spawn Application State, this controls the state of the application
    commands.spawn(RoundColorState::default());

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
        Troop,
        Player,
        TilePos { x: 10, y: 10 },
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
    commands.entity(tilemap_entity).insert((TilemapBundle {
        grid_size,
        size: map_size,
        storage: tile_storage,
        map_type,
        texture: TilemapTexture::Single(texture_handle),
        tile_size,
        anchor: TilemapAnchor::Center,
        ..Default::default()
    },PlayZoneTilemap));

    commands
        .entity(tilemap_entity)
        .insert(Transform::from_xyz(0., 0., LAYER_TILEMAP as f32));
}

fn update(commands: Commands, time: Res<Time>, global_turn_state: Query<&mut GlobalTurnState>, mut global_app_state: Query<&mut GlobalApplicationState>, asset_server: Res<AssetServer>, mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>) {
    let mut value = gameplay(commands, time, global_turn_state, asset_server, texture_atlas_layouts);
    for app_state in &mut global_app_state {
        match app_state.application_state {
            ApplicationState::MainMenu => main_menu(),
            ApplicationState::Gameplay => value,
            ApplicationState::EndMenu => end_menu(),
        }
    }
}

fn main_menu() {
    println!("This is the main menu!");
}

fn end_menu() {
    println!("This is the end menu!");
}

fn gameplay(commands: Commands, time: Res<Time>, query: Query<&mut GlobalTurnState>, asset_server: Res<AssetServer>, texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>) {
    logical_turn(commands, query, asset_server, texture_atlas_layouts);
}

fn spawn_enemy(tile_pos_x: u32, tile_pos_y: u32, mut commands: Commands, asset_server: Res<AssetServer>, mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>){
    
    let texture = asset_server.load("Enemy.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(16), 3, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    let animation_indices = AnimationIndices { first: 0, last: 2 };

    commands.spawn((
        Troop,
        Enemy,
        TilePos { x: tile_pos_x, y: tile_pos_y },
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
}

fn color_pick_update(mut commands: Commands, asset_server: Res<AssetServer>, mut color_state: Query<&mut RoundColorState>)
{
    let offset_up = 168.0f32;
    let offset_right = 30.0f32;

    for color_state in color_state.iter_mut() {

    // Load Spectrum Sprites
    let texture_handle_blue: bevy::prelude::Handle<Image> = asset_server.load("Spectrum-Blue-Sphere.png");
    let texture_handle_red: bevy::prelude::Handle<Image> = asset_server.load("Spectrum-Red-Sphere.png");
    let texture_handle_green: bevy::prelude::Handle<Image> = asset_server.load("Spectrum-Green-Sphere.png");
    let texture_handle_middle: bevy::prelude::Handle<Image> = asset_server.load("Arrow.png");
    let texture_handle_arrow: bevy::prelude::Handle<Image> = asset_server.load("Spectrum-Masked.png");

    if color_state.index != 0 // Red gets masked
    {
        // The colors in the spectrum
        commands.spawn((
            Sprite {
                image: texture_handle_red,
                color: Color::srgba(1.0, 0.0, 0.0, 0.7),
                ..default()
            },
            Transform::from_xyz(-offset_right, offset_up, LAYER_UI as f32),
            SpectrumElement
        ));
    }
    if color_state.index != 1
    {
        commands.spawn((
            Sprite {
                image: texture_handle_green,
                color: Color::srgba(0.0, 1.0, 0.0, 0.7),
                ..default()
            },
            Transform::from_xyz(-offset_right, offset_up, LAYER_UI as f32),
            SpectrumElement
        ));
    }
    if color_state.index != 2
    {
        commands.spawn((
            Sprite {
                image: texture_handle_blue.clone(),
                color: Color::srgba(0.0, 0.0, 1.0, 0.7),
                ..default()
            },
            Transform::from_xyz(-offset_right, offset_up, LAYER_UI as f32),
            SpectrumElement
        ));
    }

    // Arrow that shows what gets masked
    let mut r= 0.0f32;
    let mut g= 0.0f32;
    let mut b= 0.0f32;
    let mut r2= 1.0f32;
    let mut g2= 1.0f32;
    let mut b2= 1.0f32;

    if color_state.index == 0{
        r = 1.0f32;
        r2 = 0.0f32;
    }
    else if color_state.index == 1{
        g = 1.0f32;
        g2 = 0.0f32;
    }
    else{
        b = 1.0f32;
        b2 = 1.0f32;
    }

    commands.spawn((
            Sprite {
                image: texture_handle_middle.clone(),
                color: Color::srgba(r, g, b, 1.),
                ..default()
            },
            Transform::from_xyz(offset_right - offset_right / 2 as f32 , offset_up, LAYER_UI as f32),
            SpectrumElement
        ));

    // Round Resulting color
    commands.spawn((
            Transform::from_xyz(offset_right, offset_up, LAYER_UI as f32),
            Sprite {
                image: texture_handle_arrow.clone(),
                color: Color::srgba(r2, g2, b2, 1.),
                ..default()
            },
            SpectrumElement
        ));
    }
}

fn logical_turn(mut commands: Commands, mut query: Query<&mut GlobalTurnState>, asset_server: Res<AssetServer>, mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>) {
    for mut turn_state_entity in query.iter_mut() {
    match turn_state_entity.turn_state {
            TurnState::ColorPick => {
                turn_state_entity.modify_state(TurnState::PlayerChange);
                return;
            },
            TurnState::PlayerChange => {
                println!("Will player get damage?");
                turn_state_entity.turn_state = TurnState::EnemySpawn;
                return;
            },
            TurnState::EnemySpawn => { 
                println!("Enemy Spawn");
                spawn_enemy(15, 15, commands, asset_server, texture_atlas_layouts);
                turn_state_entity.turn_state = TurnState::MovePlayer;
                return;
            },
            TurnState::MovePlayer => {
                
            },
            TurnState::AttackPlayer => {
                commands.spawn(button::button(&asset_server));
            },
            TurnState::MoveEnemy => {
                println!("Move Enemy")
            },
            TurnState::AttackEnemy => {
                println!("Attack Enemy")
            },
            _ => {}
    }
}
}

fn keyboard_input(keys: Res<ButtonInput<KeyCode>>, mut player_q: Query<&mut TilePos, With<Player>>, // We want the player
    tilemap_q: Query<(&TileStorage, &TilemapSize), With<PlayZoneTilemap>>, // Retrieve the tilemap and it's bundaries for safe writing
    tile_q: Query<&mut TileColor>,
){
    // Exit the application.
    if keys.just_pressed(KeyCode::Escape) {
        std::process::exit(0);
    }

    // Move The Player
    // TODO: possible PS5 controller path
    let Ok((_storage, map_size)) = tilemap_q.single() else { return; };

    for mut pos in player_q.iter_mut() {
        let mut next = *pos;

        if keys.just_pressed(KeyCode::ArrowUp)    { next.y += 1; }
        if keys.just_pressed(KeyCode::ArrowDown) { next.y = next.y.saturating_sub(1); }
        if keys.just_pressed(KeyCode::ArrowRight) { next.x += 1; }
        if keys.just_pressed(KeyCode::ArrowLeft) { next.x = next.x.saturating_sub(1); }

        let max_x = (map_size.x - 1) as i32;
        let max_y = (map_size.y - 1) as i32;

        let x = (next.x as i32).clamp(0, max_x) as u32;
        let y = (next.y as i32).clamp(0, max_y) as u32;

        *pos = TilePos{ x, y };
    }

    // Attack with player
    if keys.just_pressed(KeyCode::KeyQ) {
        crate::troop_utilities::color_player_neighbors(troop_utilities::AttackPattern::Diagonal, player_q, tilemap_q, tile_q);
    }
    else if keys.just_pressed(KeyCode::KeyW) {
        crate::troop_utilities::color_player_neighbors(troop_utilities::AttackPattern::Sides, player_q, tilemap_q, tile_q);
    }
    else if keys.just_pressed(KeyCode::KeyE) {
        crate::troop_utilities::color_player_neighbors(troop_utilities::AttackPattern::Around, player_q, tilemap_q, tile_q);
    }
    else if keys.just_pressed(KeyCode::KeyR) {
        crate::troop_utilities::color_player_neighbors(troop_utilities::AttackPattern::Ultimate, player_q, tilemap_q, tile_q);
    }

}

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: String::from("Accessing Tiles Example"),
                        mode: WindowMode::Fullscreen(MonitorSelection::Primary, VideoModeSelection ::Current),
                        resolution: (1920, 1080).into(),
                        position: WindowPosition::Centered(MonitorSelection::Primary),
                        ..Default::default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        ) // Plugin: Window
        .init_resource::<InputFocus>() // InputFocus must be set for accessibility to recognize the button.
        .add_plugins(TilemapPlugin) // Plugin: Tilemap
        .add_plugins(TiledPlugin::default()) // Plugin: Tilemap Tiled
        .add_plugins(EguiPlugin::default()) // Plugin: Egui
        .add_systems(Startup, startup)
        .add_systems(Update, keyboard_input) // Input: keyboard
        .add_systems(Update, camera::movement) // Input: camera from Bevy's official example code
        .add_systems(Update, update)
        .add_systems(Update, animate_sprite)
        .add_systems(Update, snap_troop_to_tilemap)
        .add_systems(Update, color_pick_update)
        //.add_systems(EguiPrimaryContextPass, editor::ui_example_system) // Update: Egui
        .run();
}