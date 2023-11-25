use bevy::prelude::Component;

// Just a tag to bundle with other components,
// such as translation
#[derive(Component)]
pub struct Player;

// Is this a good place for constant/static values?
impl Player {
    pub const MOVEMENT_SPEED: f32 = 500.0;
}
