use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

mod camera;

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Enemy;

enum TroopType {
    Enemy,
    Player,
}

enum TroopColor {
    Red,
    Green,
    Blue,
    Yellow,
    Magenta,
    Cyan,
}

#[derive(Component)]
struct Troop {
    position_x: u32,
    position_y: u32,
    troop_type: TroopType,
    troop_color: TroopColor,
}

impl Default for Troop {
    fn default() -> Self {
        Troop {
            position_x: 0,
            position_y: 0,
            troop_type: TroopType::Enemy,
            troop_color: TroopColor::Red,
        }
    }
}

// This controls the flow inside the application
// Application State
enum ApplicationState{
    MainMenu,
    Gameplay,
    EndMenu
}

#[derive(Component)]
struct GlobalApplicationState{
    application_state: ApplicationState
}

impl Default for GlobalApplicationState{
    fn default() -> Self{
        GlobalApplicationState{application_state: ApplicationState::MainMenu,}
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

fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    
    // Spawn Camera
    commands.spawn(Camera2d);

    // Spawn Application State, this controls the state of the application
    commands.spawn(GlobalApplicationState::default());

    // Spawn Turn State, this controls the turn based mechanics of the game.
    commands.spawn(GlobalTurnState::default());

    // Spawn Player: player, sprite and transform components.
    commands.spawn((Player, Transform::from_xyz(0., 0., 1.), Sprite::from_image(asset_server.load("character.png"))));

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
            // Here we let the tile storage component know what tiles we have.
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
}

// Query the global game state to keep track of current state and implement
// turn based logic.
fn logical_turn(mut commands: Commands, time: Res<Time>, mut query: Query<&mut GlobalTurnState>) {

}

fn update(mut commands: Commands, time: Res<Time>, mut query: Query<&mut GlobalApplicationState>) {
    for mut appState in &mut query
    {
        match appState.application_state{
            ApplicationState::MainMenu => main_menu(),
            ApplicationState::Gameplay => gameplay(),
            ApplicationState::EndMenu => end_menu(),
        }
    }
}

fn main_menu()
{
    println!("This is the main menu!");
}

fn gameplay()
{
    println!("This is the gameplay!");
}

fn end_menu()
{
    println!("This is the end menu!");
}

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: String::from("Accessing Tiles Example"),
                        ..Default::default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugins(TilemapPlugin)
        .add_systems(Startup, startup)
        .add_systems(Update, camera::movement)
        .add_systems(Update, update)
        .add_systems(Update, logical_turn)
        .run();
}
