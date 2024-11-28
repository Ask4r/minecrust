use bevy::prelude::*;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;

pub struct RandomPlugin;

impl Plugin for RandomPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

#[derive(Resource)]
pub struct RandomSource(pub ChaCha8Rng);

fn setup(mut commands: Commands) {
    let seeded_rng = ChaCha8Rng::seed_from_u64(19878367467712);
    commands.insert_resource(RandomSource(seeded_rng));
}
