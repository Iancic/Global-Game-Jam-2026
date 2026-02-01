use bevy::prelude::*;
use rand::Rng;

#[derive(Component)]
pub struct PlayZoneTilemap;

// Sole reason for this component is this
// https://bevy.org/learn/errors/b0001/
#[derive(Component)]
pub struct RoundColorState2;

#[derive(Component)]
pub struct RoundColorState {
    pub index: i32, // 0 - Red is masked, 1 - Green is masked, 2 - Blue is masked
}

impl RoundColorState {
    pub fn asign_random_color(&mut self) {
        let mut rng = rand::rng();
        self.index = rng.random_range(0..3); // 0 - red, 1 - green, 2 - blue
    }
}

impl Default for RoundColorState {
    fn default() -> Self {
        RoundColorState { index: 0 }
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

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct Troop;

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

#[derive(Clone, Copy)]
pub enum TurnState {
    ColorPick,
    PlayerChange,
    EnemySpawn,
    MovePlayer,
    AttackPlayer,
    MoveEnemy,
    AttackEnemy,
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

impl GlobalTurnState {
    pub fn modify_state(&mut self, new_state: TurnState) {
        self.turn_state = new_state;
    }
}
