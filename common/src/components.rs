use bevy::prelude::*;

#[derive(Component)]
pub struct DancingGrandpa {
    pub dancer_id: usize,
    pub base_position: Vec3,
    pub current_frame: usize,
    pub frame_timer: f32,
}

#[derive(Component)]
pub struct FadingOut {
    pub initial_alpha: f32,
    pub fade_progress: f32,
}

#[derive(Component)]
pub struct AudioSource;