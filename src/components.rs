use bevy::prelude::*;
use rand::Rng;

#[derive(Component)]
pub struct PlayZoneTilemap;

#[derive(Component)]
pub struct RoundColorState
{
    pub index: i32, // 0 - Red is masked, 1 - Green is masked, 2 - Blue is masked
}

impl RoundColorState{
    pub fn asign_random_color(&mut self)
    {
        let mut rng = rand::rng();
        self.index = rng.random::<i32>() as i32 % 3; // 0 - red, 1 - green, 2 - blue
    }
}

impl Default for RoundColorState{
    fn default() -> Self {
        RoundColorState{index: 0}
    }
}

#[derive(Component)]
pub struct SpectrumElement;

#[derive(Component)]
pub struct AnimationIndices {
    pub first: usize,
    pub last: usize,
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

pub fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(&AnimationIndices, &mut AnimationTimer, &mut Sprite)>,
) {
    for (indices, mut timer, mut sprite) in &mut query {
        timer.tick(time.delta());

        if timer.just_finished()
            && let Some(atlas) = &mut sprite.texture_atlas
        {
            atlas.index = if atlas.index == indices.last {
                indices.first
            } else {
                atlas.index + 1
            };
        }
    }
}

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct Troop;

// This controls the flow inside the application
// Application State
pub enum ApplicationState {
    MainMenu,
    Gameplay,
    EndMenu,
}

#[derive(Component)]
pub struct GlobalApplicationState {
    pub application_state: ApplicationState,
}

impl Default for GlobalApplicationState {
    fn default() -> Self {
        GlobalApplicationState {
            application_state: ApplicationState::Gameplay,
        }
    }
}

// This controls the flow inside the gameplay stage
// Turn State

#[derive(Clone, Copy)]
pub enum TurnState {
    ColorPick,    // a color mask gets picked
    PlayerChange, // change player color and change for state
    EnemySpawn,   // spawn enemies
    MovePlayer,   // move player
    AttackPlayer, // player attacks
    MoveEnemy,  // move enemies
    AttackEnemy,  // enemy attacks
}

#[derive(Component)]
pub struct GlobalTurnState {
    pub turn_state: TurnState,
}

impl Default for GlobalTurnState {
    fn default() -> Self {
        GlobalTurnState {
            turn_state: TurnState::ColorPick,
        }
    }
}

impl GlobalTurnState{
    pub fn modify_state(&mut self, new_state: TurnState)
    {
        self.turn_state = new_state;
    }
}
