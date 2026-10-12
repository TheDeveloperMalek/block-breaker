use bevy::prelude::Resource;

#[derive(Resource, Default)]
pub(crate) struct Score {
    pub(crate) points: u32,
    pub(crate) best_points: u32,
}
