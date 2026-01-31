use bevy::app::AppExit;
use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use bevy_egui::{EguiPlugin, EguiPrimaryContextPass};
use bevy::window::WindowMode;
use bevy::ecs::event::*;

mod camera;
mod editor;

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Enemy;

#[derive(Component)]
struct Troop;

// This controls the flow inside the application
// Application State
enum ApplicationState {
    MainMenu,
    Gameplay,
    EndMenu,
}

#[derive(Component)]
struct GlobalApplicationState {
    application_state: ApplicationState,
}

impl Default for GlobalApplicationState {
    fn default() -> Self {
        GlobalApplicationState {
            application_state: ApplicationState::MainMenu,
        }
    }
}

// This controls the flow inside the gameplay stage
// Turn State
enum TurnState {
    ColorPick,    // a color mask gets picked
    PlayerChange, // change player color and change for state
    EnemySpawn,   // spawn enemies
    MoveEnemy,    // move enemies
    MovePlayer,   // move player
    AttackPlayer, // player attacks
    AttackEnemy,  // enemy attacks
}

#[derive(Component)]
struct GlobalTurnState {
    turn_state: TurnState,
}

impl Default for GlobalTurnState {
    fn default() -> Self {
        GlobalTurnState {
            turn_state: TurnState::ColorPick,
        }
    }
}

const TILEMAP_SIDE_LENGHT: u32 = 16;
const LAYER_PLAYER: u8 = 2;
const LAYER_TILEMAP: u8 = 1;

fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    
    

    // Spawn Camera
    commands.spawn(Camera2d);

    // Spawn Application State, this controls the state of the application
    commands.spawn(GlobalApplicationState::default());

    // Spawn Turn State, this controls the turn based mechanics of the game.
    commands.spawn(GlobalTurnState::default());

    // SPawn Spectrum
    let texture_handle_blue: bevy::prelude::Handle<Image> = asset_server.load("Spectrum-Blue-Sphere.png");
    let texture_handle_red: bevy::prelude::Handle<Image> = asset_server.load("Spectrum-Red-Sphere.png");
    let texture_handle_green: bevy::prelude::Handle<Image> = asset_server.load("Spectrum-Green-Sphere.png");

    commands.spawn((
        Sprite{
            image: texture_handle_red.clone(),
            // Alpha channel of the color controls transparency.
            color: Color::srgba(1.0, 0.0, 0.0, 0.7),
            ..default()
        },
        Transform::from_xyz(0.0, 1.3, 3.2),
    ));
    commands.spawn((
        Sprite {
            image: texture_handle_green,
            color: Color::srgba(0.0, 1.0, 0.0, 0.7),
            ..default()
        },
        Transform::from_xyz(0.0, 1.3, 3.2),
    ));
    commands.spawn((
        Sprite {
            image: texture_handle_blue.clone(),
            color: Color::srgba(0.0, 0.0, 1.0, 0.7),
            ..default()
        },
        Transform::from_xyz(0.0, 1.3, 3.2),
    ));
    

    // Spawn Player: troop, player, sprite and transform components.
    commands.spawn((
        Troop,
        Player,
        TilePos { x: 10, y: 10 },
        Transform::from_xyz(0., 0., LAYER_PLAYER as f32),
        Sprite::from_image(asset_server.load("character.png")),
    ));

    // Tilemap
    // From bevy_ecs_tilemap/accesing_tiles examples
    // Image Asset
    let texture_handle: Handle<Image> = asset_server.load("tilemap_packed.png");

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
                commands.entity(tile_entity).insert(TileTextureIndex(1));
            }
        }
    }

    // This is the size of each individual tiles in pixels.
    let tile_size = TilemapTileSize {
        x: TILEMAP_SIDE_LENGHT as f32,
        y: TILEMAP_SIDE_LENGHT as f32,
    };
    let grid_size = tile_size.into();

    // Spawns a tilemap.
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
    },));

    commands
        .entity(tilemap_entity)
        .insert(Transform::from_xyz(0., 0., LAYER_TILEMAP as f32));
}

fn update(commands: Commands, time: Res<Time>, mut query: Query<&mut GlobalApplicationState>) {
    for app_state in &mut query {
        match app_state.application_state {
            ApplicationState::MainMenu => main_menu(),
            ApplicationState::Gameplay => gameplay(),
            ApplicationState::EndMenu => end_menu(),
        }
    }
}

fn main_menu() {
    println!("This is the main menu!");
}

fn gameplay() {
    println!("This is the gameplay!");
}

fn logical_turn(mut commands: Commands, time: Res<Time>, mut query: Query<&mut GlobalTurnState>) {}

// GAMEPLAY LOGIC: I should make this as plugin

// Credit: snapping idea to center in world from Codex 5.2
fn snap_troop_to_tilemap(
    mut player_q: Query<(&TilePos, &mut Transform), With<Troop>>, // All entities with transform, player and position
    tilemap_q: Query<(
        &TilemapSize,
        &TilemapGridSize,
        &TilemapTileSize,
        &TilemapType,
        &TilemapAnchor,
    )>, // Necessary for center in world
) {
    // Tries to query for an entity with the data required to snap.
    // If there is none we have nothing to store, doesn't pass the Ok check move on.
    let Ok((map_size, grid_size, tile_size, map_type, anchor)) = tilemap_q.single() else {
        println!("No tilemap.");
        return;
    };

    for (tile_pos, mut transform) in player_q.iter_mut() {
        // Retrieve center from the tilemap:
        let center = tile_pos.center_in_world(map_size, grid_size, tile_size, map_type, anchor);

        // Change the transform from the player
        transform.translation.x = center.x;
        transform.translation.y = center.y;
        transform.translation.z = LAYER_PLAYER as f32;
    }
}

// Credit: inspiration for retrieving the neighboring tiles using Codex 5.2
fn color_player_neighbors(
    mut player_q: Query<&mut TilePos, With<Player>>, // We want the player
    tilemap_q: Query<(&TileStorage, &TilemapSize)>, // Retrieve the tilemap and it's bundaries for safe writing
    mut tile_q: Query<&mut TileColor>,
) {
    let Ok((storage, map_size)) = tilemap_q.single() else {
        println!("No tilemap.");
        return;
    };
    // Now that I have where the player is in tile position,
    // I make an array with all positions around him
    for pos in player_q.iter(){
        
        let neighbors = [
            *pos, // Where I am now
            TilePos{x: pos.x + 1, y: pos.y}, // Right side
            TilePos{x: pos.x.wrapping_sub(1), y: pos.y}, // Left side
            TilePos{x: pos.x, y: pos.y + 1}, // Top side
            TilePos{x: pos.x, y: pos.y.wrapping_sub(1)}, // Bottom side
        ];
        // With these new positions I can iterate them one by one and if not outside the bounds color them
        for tile in neighbors {
            if tile.x < map_size.x && tile.y < map_size.y { // the neighbouring tile can be outside that is invalid
                if let Some(tile_entity) = storage.get(&tile){ // retrieve entity
                    if let Ok(mut color) = tile_q.get_mut(tile_entity) // If there is a retrievable tile
                    {
                        *color = TileColor(Color::srgba(1.0, 0.0, 0.0, 1.0)); // TODO: red for now but make it based on
                    }
                }
            }
        }
    }
}

fn end_menu() {
    println!("This is the end menu!");
}

// helper
fn clamp_tile(pos: TilePos, size: TilemapSize) -> TilePos {
    let max_x = (size.x - 1) as i32;
    let max_y = (size.y - 1) as i32;

    let x = (pos.x as i32).clamp(0, max_x) as u32;
    let y = (pos.y as i32).clamp(0, max_y) as u32;

    TilePos { x, y }
}

fn keyboard_input(keys: Res<ButtonInput<KeyCode>>,
    mut player_q: Query<&mut TilePos, With<Player>>, // We want the player
    tilemap_q: Query<(&TileStorage, &TilemapSize)>, // Retrieve the tilemap and it's bundaries for safe writing
    mut tile_q: Query<&mut TileColor>,
){
    // Move The Player
    // TODO: possible PS5 controller path
    if keys.just_pressed(KeyCode::Escape) {
        std::process::exit(0);
    }

    let Ok((_storage, map_size)) = tilemap_q.single() else { return; };

    for mut pos in player_q.iter_mut() {
        let mut next = *pos;

        if keys.just_pressed(KeyCode::ArrowUp)    { next.y += 1; }
        if keys.just_pressed(KeyCode::ArrowDown) { next.y = next.y.saturating_sub(1); }
        if keys.just_pressed(KeyCode::ArrowRight){ next.x += 1; }
        if keys.just_pressed(KeyCode::ArrowLeft) { next.x = next.x.saturating_sub(1); }

        *pos = clamp_tile(next, *map_size);
    }

    if keys.just_pressed(KeyCode::Space) {
        color_player_neighbors(player_q, tilemap_q, tile_q);
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
        .add_plugins(EguiPlugin::default()) // Plugin: Egui
        .add_plugins(TilemapPlugin) // Plugin: Tilemap
        .add_systems(Startup, startup) // Startup
        .add_systems(Update, keyboard_input) // Input: keyboard
        .add_systems(Update, camera::movement) // Input: camera from Bevy's official example code
        .add_systems(Update, update)
        .add_systems(Update, logical_turn) // Update: Turn Based Logic, Gameplay
        .add_systems(Update, snap_troop_to_tilemap) // Update: Troop Snap, Gameplay
        .add_systems(EguiPrimaryContextPass, editor::ui_example_system) // Update: Egui
        .run();
}