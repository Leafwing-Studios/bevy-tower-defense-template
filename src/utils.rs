use bevy::prelude::*;

#[derive(Debug)]
pub struct NotInWindowError;

/// Returns the mouse position in world coordinates
pub fn fetch_mouse_coordinates(
    windows: &Windows,
    camera_transform: &Transform,
) -> Result<Vec4, NotInWindowError> {
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
        return Ok(camera_transform.compute_matrix() * p.extend(0.0).extend(1.0))
    } else {
		return Err(NotInWindowError)
	}
}

/// Returns the tile coordinates at the given world coordinates
pub fn tile_from_world_coordinates(world_position: Vec4) -> UVec2 {
	// FIXME: add logic
	UVec2::new(1,1)
}
