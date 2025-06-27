use bevy::prelude::*;
use crate::config::DancingGrandpaConfig;

#[derive(Resource)]
pub enum GameState {
    Loading,
    Playing,
    FadingOut,
    Silence,
}

#[derive(Resource)]
pub struct AnimationTimer(pub Timer);

#[derive(Resource)]
pub struct FadeTimer(pub Timer);

#[derive(Resource)]
pub struct RestartTimer(pub Timer);

#[derive(Resource)]
pub struct Config(pub DancingGrandpaConfig);

#[derive(Resource)]
pub struct LoadedImages(pub Vec<Handle<Image>>);

#[derive(Resource)]
pub struct LoadedAudio(pub Handle<AudioSource>);

#[derive(Resource)]
pub struct AudioInstance(pub Option<Entity>);