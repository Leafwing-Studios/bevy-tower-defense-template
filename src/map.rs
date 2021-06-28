use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use crate::construction::Buildable;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup_map.system())
            .init_resource::<TileSize>()
            .add_system(color_buildable_tiles.system());
    }
}

/// Controls the size of the tiles on the map in pixels
/// 
/// Note that the x and y coordinates of hexagonal tiles are not identical
/// This is also the size (in pixels) of the tile sprite
pub struct TileSize(pub Vec2);

impl Default for TileSize {
    fn default() -> Self {
        TileSize(Vec2::new(15.0, 17.0))
    }
}

fn setup_map(
    mut commands: Commands,

    tile_size: Res<TileSize>,
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
        UVec2::new(3, 3),
        UVec2::new(8, 8),
        tile_size.0,
        Vec2::new(105.0, 17.0),
    );

    layer_settings.mesh_type = TilemapMeshType::Hexagon(HexType::Row);

    // Creates a new layer builder with a layer entity.
    let (mut layer_builder, _) = LayerBuilder::new(&mut commands, layer_settings, 0u16, 0u16);

    layer_builder.set_all(TileBundle::default());

    // Add Buildable to tiles
    layer_builder.for_each_tiles(
        |tile_entity, _| {
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
        .insert(Transform::default())
        .insert(GlobalTransform::default());
}

/// Changes the color of tiles based on whether or not they are buildable
fn color_buildable_tiles(
    mut query: Query<(&Buildable, &mut Tile, &UVec2), Changed<Buildable>>,
    mut map_query: MapQuery,
) {
    for (buildable, mut tile, position) in query.iter_mut(){
        if !buildable.0 {
            tile.texture_index = 4;
        }
        // This is needed so the chunk will update the mesh, changing the displayed graphics
        map_query.notify_chunk_for_tile(*position, 0u16, 0u16);
    }
}
