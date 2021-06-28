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

// FIXME: this is very inefficient
/// Returns the tile coordinates at the given world coordinates
pub fn tile_from_world_coordinates(
    world_position: Vec2, 
    tile_size: Vec2
) -> Option<UVec2> {
    
    let hex_basis = Mat2::from_cols(Vec2::new(3f32.sqrt()/3.0, 0.0), Vec2::new(-1.0/3.0, 2.0/3.0));
    let tile_offset = tile_size * -0.5;
    
    let hex_basis_position = (hex_basis * (world_position + tile_offset)) / (tile_size.y * 0.5);

    let tile_position = hex_round(hex_basis_position);
    if tile_position.min_element() < 0.0 {
        return None;
    } else {
        return Some(tile_position.as_u32());
    }
}

fn hex_round(coords: Vec2) -> Vec2 {
    let cube_coords = coords.extend( -1.0 * coords.x - coords.y);
    
    let mut rounded_cube_coords = cube_coords.round();
    let coords_diff = rounded_cube_coords - cube_coords;
    
    if coords_diff.x > coords_diff.y && coords_diff.x > coords_diff.z {
        rounded_cube_coords.x = -1.0 * rounded_cube_coords.y - rounded_cube_coords.z;
    } else if coords_diff.y > coords_diff.z {
        rounded_cube_coords.y = -1.0 * rounded_cube_coords.x - rounded_cube_coords.z;
    } else {
        rounded_cube_coords.z = -1.0 * rounded_cube_coords.x - rounded_cube_coords.y;
    }

    return rounded_cube_coords.truncate();
}