use crate::prelude::*;

pub const ARENA_WIDTH: f32 = 1280.0;
pub const ARENA_HEIGHT: f32 = 800.0;

#[derive(Debug, Resource)]
pub struct Arena {
    pub asteroid_spawn_timer: Timer,
    pub score: u32,
}

pub struct ArenaPlugin;

impl Plugin for ArenaPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::GameCreate), spawn_arena)
            .add_systems(Update, movement.run_if(in_state(AppState::GameRunning)));
    }
}

fn spawn_arena(mut commands: Commands) {
    commands.insert_resource(Arena {
        asteroid_spawn_timer: Timer::from_seconds(5.0, TimerMode::Once),
        score: 0,
    });

    // Physics configuration without gravity
    commands.insert_resource(Gravity::ZERO);
}

fn interval_calc(value: f32, vel: f32, min: f32, max: f32) -> f32 {
    if value < min && vel < 0.0 {
        max
    } else if value > max && vel > 0.0 {
        min
    } else {
        value
    }
}

fn movement(mut query: Query<(&LinearVelocity, &mut Position)>) {
    for (linvel, mut position) in query.iter_mut() {
        let mut x = position.x;
        let mut y = position.y;
        // Wrap around screen edges
        let half_width = ARENA_WIDTH / 2.0;
        let half_height = ARENA_HEIGHT / 2.0;
        x = interval_calc(x, linvel.x, -half_width, half_width);
        y = interval_calc(y, linvel.y, -half_height, half_height);
        position.x = x;
        position.y = y;
    }
}
