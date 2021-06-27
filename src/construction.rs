use crate::utils::{fetch_mouse_coordinates, tile_from_world_coordinates};
use crate::MainCamera;
use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

/// Systems and resources for selecting towers, paying for them, and placing them on the grid
pub struct ConstructionPlugin;

impl Plugin for ConstructionPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<Gold>()
			// The tower type that is selected to be built
            .insert_resource(TowerType::Basic)
            .add_system(build_tower.system());
    }
}

/// Marker component for towers
#[derive(Clone, Copy)]
struct Tower;

/// Gold cost to construct
#[derive(Clone)]
struct Cost(u32);

/// Basic statistics for attacks made by towers
pub struct AttackStats {
    /// Damage that the tower deals per hit
    damage: u32,
    /// Range (in units of the standard Transform)
    range: f32,
    /// Timer that manages when attacks fire
    attack_timer: Timer,
}

/// The list of all tower types
#[derive(Clone, Copy)]
enum TowerType {
    Basic,
}

/// Bundle for creating tower entities
#[derive(Bundle)]
struct TowerBundle {
    tower: Tower,
    cost: Cost,
    attack_stats: AttackStats,
}

impl TowerBundle {
    /// Create a new tower of the given type with the appropriate value
    fn new(tower_type: TowerType) -> Self {
        match tower_type {
            TowerType::Basic => TowerBundle {
                tower: Tower,
                cost: Cost(20),
                attack_stats: AttackStats {
                    damage: 10,
                    range: 100.0,
                    attack_timer: Timer::from_seconds(10.0, true),
                },
            },
        }
    }
}

/// The amount of gold the player has, used to build towers
struct Gold(u32);

impl Default for Gold {
    fn default() -> Self {
        Gold(500)
    }
}

/// Controls whether a tile can be built on or not
pub struct Buildable(bool);

impl Default for Buildable {
	fn default() -> Self{
		Buildable(true)
	}
}

/// System to build a tower at the cursor when the "B" key is pressed
fn build_tower(
    mut commands: Commands,
    map: MapQuery,
    
    keyboard_input: Res<Input<KeyCode>>,
    windows: Res<Windows>,
    selected_tower_type: Res<TowerType>,
    mut gold: ResMut<Gold>,
    
    camera_query: Query<&Transform, With<MainCamera>>,
    mut tile_query: Query<&mut Buildable, With<Tile>>,
) {
    // Only build a tower when "B" is pressed
    if keyboard_input.just_pressed(KeyCode::B) {
        // Determine cost of selected tower
        let tower_bundle = TowerBundle::new(*selected_tower_type);
        let cost = &tower_bundle.cost;

        // Check cost
        if gold.0 < cost.0 {
            info!("You do not have enough gold to build this tower.");
            return;
        }

        // Select tile under cursor
		let camera_transform = camera_query.single().unwrap();
        let mouse_position = fetch_mouse_coordinates(&*windows, camera_transform);
		
		if mouse_position.is_err(){
			info!("Mouse not in main window.");
			return;
		}

		let tile_coordinates = tile_from_world_coordinates(mouse_position.unwrap());

		// FIXME: lookup current map from resource
		// FIXME: use enum value for appropriate layer
        let tile_entity = map.get_tile_entity(tile_coordinates, 0 as u16, 0 as u16);

        // Check that a tile exists under the cursor
        if tile_entity.is_err() {
            info!("No tile under cursor.");
            return;
        }

        // Check that the tile can be built on
        let mut buildable = tile_query.get_mut(tile_entity.unwrap()).unwrap();
        if !buildable.0 {
            info!("You cannot build there.");
            return;
        }

        // Pay for the tower
        gold.0 -= tower_bundle.cost.0;

        // Set the tile as unbuildable
        buildable.0 = false;

        // Spawn the tower entity
        commands.spawn_bundle(tower_bundle);
    }
}
