const FRAC_1_3: f32 = 1.0 / 3.0;
const FRAC_2_3: f32 = 2.0 / 3.0;
// Padding used to prevent UV's overlapping.
const UV_PADDING: f32 = 0.0001;

// NOTE: Following UVs are written for bevy's Cuboid mesh indices

// NOTE: (0.0, 0.0) = Top-Left in UV mapping,
// (1.0, 1.0) = Bottom-Right in UV mapping

/// Treats texture image as single texture for each side.
#[rustfmt::skip]
pub fn uv_single_side_alignment() -> Vec<[f32; 2]> {
    vec![
        // Assigning the UV coords for the forward side.
        [1.0, 1.0], [0.0, 1.0], [0.0, 0.0], [1.0, 0.0],
        // Assigning the UV coords for the back side.
        [0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0],
        // Assigning the UV coords for the left side.
        [0.0, 1.0], [0.0, 0.0], [1.0, 0.0], [1.0, 1.0],
        // Assigning the UV coords for the right side.
        [0.0, 1.0], [0.0, 0.0], [1.0, 0.0], [1.0, 1.0],
        // Assigning the UV coords for the top side.
        [0.0, 1.0], [0.0, 0.0], [1.0, 0.0], [1.0, 1.0],
        // Assigning the UV coords for the bottom side.
        [0.0, 1.0], [0.0, 0.0], [1.0, 0.0], [1.0, 1.0],
    ]
}

/// Treats texture image as 3 square textures stacked vertically:
/// first for 4 sides (left, right, forward and back),
/// second for top, third for bottom.
#[rustfmt::skip]
pub fn uv_sides_top_bottom_alignment() -> Vec<[f32; 2]> {
    vec![
        // Assigning the UV coords for the forward side.
        [1.0, FRAC_1_3 - UV_PADDING], [0.0, FRAC_1_3 - UV_PADDING], [0.0, 0.0], [1.0, 0.0],
        // Assigning the UV coords for the back side.
        [0.0, 0.0], [1.0, 0.0], [1.0, FRAC_1_3 - UV_PADDING], [0.0, FRAC_1_3 - UV_PADDING],
        // Assigning the UV coords for the left side.
        [0.0, FRAC_1_3 - UV_PADDING], [0.0, 0.0], [1.0, 0.0], [1.0, FRAC_1_3 - UV_PADDING],
        // Assigning the UV coords for the right side.
        [0.0, FRAC_1_3 - UV_PADDING], [0.0, 0.0], [1.0, 0.0], [1.0, FRAC_1_3 - UV_PADDING],
        // Assigning the UV coords for the top side.
        [0.0, FRAC_2_3 - UV_PADDING], [0.0, FRAC_1_3 + UV_PADDING], [1.0, FRAC_1_3 + UV_PADDING], [1.0, FRAC_2_3 - f32::EPSILON],
        // Assigning the UV coords for the bottom side.
        [0.0, 1.0], [0.0, FRAC_2_3 + UV_PADDING], [1.0, FRAC_2_3 + UV_PADDING], [1.0, 1.0],
    ]
}
