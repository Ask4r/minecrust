use bevy::{
    asset::LoadState,
    core_pipeline::Skybox,
    image::{ImageSampler, ImageSamplerDescriptor},
    prelude::*,
    render::render_resource::{TextureViewDescriptor, TextureViewDimension},
};

#[derive(Resource)]
pub struct SkyboxCubemap {
    pub image: Handle<Image>,
    pub loaded: bool,
}

pub fn reinterpret_cubemap(
    asset_server: Res<AssetServer>,
    mut images: ResMut<Assets<Image>>,
    mut cubemap: ResMut<SkyboxCubemap>,
    mut skyboxes_query: Query<&mut Skybox>,
) {
    if cubemap.loaded || !matches!(asset_server.load_state(&cubemap.image), LoadState::Loaded) {
        return;
    }

    cubemap.loaded = true;
    let Some(image) = images.get_mut(&cubemap.image) else {
        error_once!("Could not load cubemap image.");
        return;
    };

    if image.texture_descriptor.array_layer_count() == 1 {
        image.reinterpret_stacked_2d_as_array(image.height() / image.width());
        image.sampler = ImageSampler::Descriptor(ImageSamplerDescriptor::nearest());
        image.texture_view_descriptor = Some(TextureViewDescriptor {
            dimension: Some(TextureViewDimension::Cube),
            ..default()
        });
    }

    // Set all skybox images to the new array texture
    for mut skybox in &mut skyboxes_query {
        skybox.image = cubemap.image.clone();
    }
}
