use bevy::prelude::*;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        // Selecting towers, paying for them, and placing them on the grid
        .add_plugin(ConstructionPlugin)
        // Towers attacking enemies
        .add_plugin(CombatPlugin)
        // Enemy behavior and stats
        .add_plugin(EnemyPlugin)
        // Spawning enemies in waves, losing lives
        .add_plugin(PacingPlugin)
        .run();
}
