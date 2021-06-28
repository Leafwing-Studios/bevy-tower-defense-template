use bevy::{math::{Mat2}, prelude::*};

#[derive(Debug)]
pub struct NotInWindowError;

/// Returns the mouse position in world coordinates
pub fn fetch_mouse_coordinates(
    windows: &Windows,
    camera_transform: &Transform,
) -> Result<Vec2, NotInWindowError> {
    // Get the primary window
    let window = windows.get_primary().unwrap();

    // Check if the cursor is in the primary window
    if let Some(pos) = window.cursor_position() {
        // Get the size of the window
        let size = Vec2::new(window.width() as f32, window.height() as f32);

        // The default orthographic projection is in pixels from the center;
        // just undo the translation
        let p = pos - size / 2.0;

        // Apply the camera transform
        let transform_vec4 = camera_transform.compute_matrix() * p.extend(0.0).extend(1.0);
        // We only care about the first two fields
        let coordinates = Vec2::new(transform_vec4.x, transform_vec4.y); 

        return Ok(coordinates)
    } else {
		return Err(NotInWindowError)
	}
}

// FIXME: this is inefficient
// Also this should probably use Result<T, E>, not Option<T>
/// Returns the tile coordinates at the given world coordinates
///
/// Based on the pixel-to-hex algorithm described here: https://www.redblobgames.com/grids/hexagons/#pixel-to-hex
pub fn tile_from_world_coordinates(
    world_position: Vec2, 
    tile_size: Vec2
) -> Option<UVec2> {
    
    // Basis vectors for axial hex coordinates
    let hex_basis = Mat2::from_cols(Vec2::new(3f32.sqrt()/3.0, 0.0), Vec2::new(-1.0/3.0, 2.0/3.0));
    // Offsets tile position so that (0,0) is the center of the (0,0) tile
    let tile_offset = tile_size * -0.5;
    
    let hex_basis_position = (hex_basis * (world_position + tile_offset)) / (tile_size.y * 0.5);

    let tile_position = hex_round(hex_basis_position);

    // Need to check for negative values here. Simply casting to UVec2 will not do
    // UVec2::as_u32() converts any negative values to 0
    if tile_position.min_element() < 0.0 {
        return None;
    } else {
        return Some(tile_position.as_u32());
    }
}

/// Takes fractional axial coordinates and "rounds" them to the nearest hex center
///
/// Based on the hex rounding algorithm described here: https://www.redblobgames.com/grids/hexagons/#rounding
fn hex_round(coords: Vec2) -> Vec2 {
    // Convert to cube coodinates
    let cube_coords = coords.extend( -1.0 * coords.x - coords.y);
    
    let mut rounded_cube_coords = cube_coords.round();
    let coords_diff = rounded_cube_coords - cube_coords;
    
    // Preserve coodinate invariants
    if coords_diff.x > coords_diff.y && coords_diff.x > coords_diff.z {
        rounded_cube_coords.x = -1.0 * rounded_cube_coords.y - rounded_cube_coords.z;
    } else if coords_diff.y > coords_diff.z {
        rounded_cube_coords.y = -1.0 * rounded_cube_coords.x - rounded_cube_coords.z;
    } else {
        rounded_cube_coords.z = -1.0 * rounded_cube_coords.x - rounded_cube_coords.y;
    }

    // Convert back to axial
    return rounded_cube_coords.truncate();
}