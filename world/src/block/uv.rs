const ONE_THIRD: f32 = 1.0 / 3.0;
const TWO_THIRDS: f32 = 2.0 / 3.0;

pub enum TextureUVAlignment {
    Single,
    SideTopBottom,
}

impl TextureUVAlignment {
    #[rustfmt::skip]
    pub fn float_uv_vec(&self) -> Vec<[f32; 2]> {
        match self {
            Self::Single => vec![
                // Assigning the UV coords for the top side.
                [0.0, 1.0], [0.0, 0.0], [1.0, 0.0], [1.0, 1.0],
                // Assigning the UV coords for the bottom side.
                [0.0, 1.0], [0.0, 0.0], [1.0, 0.0], [1.0, 1.0],
                // Assigning the UV coords for the right side.
                [0.0, 1.0], [0.0, 0.0], [1.0, 0.0], [1.0, 1.0],
                // Assigning the UV coords for the left side.
                [0.0, 1.0], [0.0, 0.0], [1.0, 0.0], [1.0, 1.0],
                // Assigning the UV coords for the back side.
                [0.0, 1.0], [0.0, 0.0], [1.0, 0.0], [1.0, 1.0],
                // Assigning the UV coords for the forward side.
                [0.0, 1.0], [0.0, 0.0], [1.0, 0.0], [1.0, 1.0],
            ],
            // Set-up UV coordinates to point to the upper (V < 0.5),
            // "dirt+grass" part of the texture.
            Self::SideTopBottom => vec![
                // Assigning the UV coords for the top side.
                [0.0, TWO_THIRDS], [0.0, ONE_THIRD], [1.0, ONE_THIRD], [1.0, TWO_THIRDS],
                // Assigning the UV coords for the bottom side.
                [0.0, 1.0], [0.0, TWO_THIRDS], [1.0, TWO_THIRDS], [1.0, 1.0],
                // Assigning the UV coords for the right side.
                [1.0, ONE_THIRD], [0.0, ONE_THIRD], [0.0, 0.0], [1.0, 0.0],
                // Assigning the UV coords for the left side.
                [1.0, ONE_THIRD], [0.0, ONE_THIRD], [0.0, 0.0], [1.0, 0.0],
                // Assigning the UV coords for the back side.
                [0.0, ONE_THIRD], [0.0, 0.0], [1.0, 0.0], [1.0, ONE_THIRD],
                // Assigning the UV coords for the forward side.
                [0.0, ONE_THIRD], [0.0, 0.0], [1.0, 0.0], [1.0, ONE_THIRD],
            ],
        }
    }
}
