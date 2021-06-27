use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use rand::{distributions::{Distribution, Uniform}, thread_rng};
use crate::construction::Buildable;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup_map.system());
    }
}

fn setup_map(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut map_query: MapQuery,
) {
    let texture_handle = asset_server.load("pointy_hex_tiles.png");
    let material_handle = materials.add(ColorMaterial::texture(texture_handle));

    // Create map entity and component:
    let map_entity = commands.spawn().id();
    let mut map = Map::new(0u16, map_entity);

    // Configure settings for layer
    let mut layer_settings = LayerSettings::new(
        UVec2::new(2, 2),
        UVec2::new(8, 8),
        Vec2::new(15.0, 17.0),
        Vec2::new(105.0, 17.0),
    );

    layer_settings.mesh_type = TilemapMeshType::Hexagon(HexType::Row);

    // Creates a new layer builder with a layer entity.
    let (mut layer_builder, _) = LayerBuilder::new(&mut commands, layer_settings, 0u16, 0u16);

    // Add tiles to the layer with random colors and the Buildable component added
    let mut random = thread_rng();
    let texture_index_range = Uniform::from(0..=2);
    layer_builder.for_each_tiles_mut(
        |tile_entity, tile_data: &mut Option<TileBundle>| {
            *tile_data = Some(
                Tile{
                    texture_index: texture_index_range.sample(&mut random),
                    ..Default::default()
                }.into()
            );
            commands.entity(tile_entity).insert(Buildable::default());
        }
    );

    // Builds the layer.
    // Note: Once this is called you can no longer edit the layer until a hard sync in bevy.
    let layer_entity = map_query.build_layer(&mut commands, layer_builder, material_handle);

    // Required to keep track of layers for a map internally.
    map.add_layer(&mut commands, 0u16, layer_entity);

    // Spawn Map
    // Required in order to use map_query to retrieve layers/tiles.
    commands
        .entity(map_entity)
        .insert(map)
        .insert(Transform::from_xyz(-128.0, -128.0, 0.0))
        .insert(GlobalTransform::default());
}
