use bevy::prelude::*;

#[derive(Resource)]
pub struct WorldParams {
    pub width: f32,
    pub height: f32,
    pub shed_location: Vec3,
    pub wheat_location: Vec3,
    pub quirks_per_chicken: usize,
    pub chicken_count: usize,
    pub fox_count: usize,
    pub nest_count: usize,
    pub chicken_required: usize,
    pub chicken_alive: usize,
    pub foxes_alive: usize,
    pub nest_locations: Vec<(f32, f32)>,
    pub chicken_hunt_interval: f32,
    pub quirk_deception_chance: f32,
}

impl WorldParams {
    pub fn apply_difficulty_preset(&mut self, preset: &DifficultyPreset) {
        self.fox_count = preset.fox_count;
        self.chicken_count = preset.chicken_count;
        self.chicken_required = preset.chicken_required;
        self.chicken_hunt_interval = preset.chicken_hunt_interval;
        self.quirk_deception_chance = preset.quirk_deception_chance;
        self.quirks_per_chicken = preset.quirks_per_chicken;
    }
}

pub struct DifficultyPreset {
    fox_count: usize,
    chicken_count: usize,
    chicken_required: usize,
    chicken_hunt_interval: f32,
    quirk_deception_chance: f32,
    quirks_per_chicken: usize,
}

// Game length should be 5 minutes for fox to win
// 10 total ~ 10% chance to hit fox
pub const EASY: DifficultyPreset = DifficultyPreset {
    fox_count: 1,
    chicken_count: 9,
    chicken_required: 6,
    chicken_hunt_interval: 90.0,
    quirk_deception_chance: 0.10,
    quirks_per_chicken: 1,
};

// Game length should be 5 minutes for fox to win
// 15 total ~ 7.5% chance to hit fox
pub const MEDIUM: DifficultyPreset = DifficultyPreset {
    fox_count: 2,
    chicken_count: 13,
    chicken_required: 9,
    chicken_hunt_interval: 75.0,
    quirk_deception_chance: 0.30,
    quirks_per_chicken: 2,
};

// Game length should be 5 minutes for fox to win
// 20 total ~ 6.6% chance to hit fox
pub const HARD: DifficultyPreset = DifficultyPreset {
    fox_count: 3,
    chicken_count: 17,
    chicken_required: 12,
    chicken_hunt_interval: 60.0,
    quirk_deception_chance: 0.50,
    quirks_per_chicken: 3,
};