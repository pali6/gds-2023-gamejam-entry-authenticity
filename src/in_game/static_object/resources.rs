use bevy::prelude::*;

#[derive(Resource)]
pub struct StaticObjectsResource {
    pub front_fence_handle: Handle<Image>,
    pub back_fence_handle: Handle<Image>,
    pub left_fence_handle: Handle<Image>,
    pub right_fence_handle: Handle<Image>,
}

impl Default for StaticObjectsResource {
    fn default() -> Self {
        Self {
            front_fence_handle: Handle::default(),
            back_fence_handle: Handle::default(),
            left_fence_handle: Handle::default(),
            right_fence_handle: Handle::default(),
        }
    }
}