use bevy::prelude::*;
use bevy_ecs_tilemap::TilemapPlugin;
mod combat;
mod construction;
mod enemy;
mod map;
mod pacing;
mod utils;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup_main_camera.system())
        // Towers attacking enemies
        // .add_plugin(CombatPlugin)
        // Selecting towers, paying for them, and placing them on the grid
        .add_plugin(construction::ConstructionPlugin)
        // Enemy behavior and stats
        // .add_plugin(EnemyPlugin)
        // Loading and manipulating the map
        .add_plugin(TilemapPlugin)
        .add_plugin(map::MapPlugin)
        // Spawning enemies in waves, losing lives
        // .add_plugin(PacingPlugin)
        .run();
}

/// Marker component for the camera that renders our non-UI graphics
pub struct MainCamera;

fn setup_main_camera(mut commands: Commands) {
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(MainCamera);
}
