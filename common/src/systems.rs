use bevy::prelude::*;
use bevy::audio::{PlaybackSettings, AudioPlayer};
use bevy::asset::LoadState;
use rand::Rng;
use std::f32::consts::PI;

#[cfg(target_arch = "wasm32")]
use web_sys::console;

use crate::{
    components::*,
    resources::*,
    config::DancingGrandpaConfig,
};

pub fn setup_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    #[cfg(target_arch = "wasm32")]
    console::log_1(&"Setup system starting".into());
    
    // Use default config
    let config = DancingGrandpaConfig::default();
    
    // Setup camera
    commands.spawn(Camera2d);
    
    #[cfg(target_arch = "wasm32")]
    console::log_1(&"Camera spawned".into());
    
    // Load grandpa images
    let grandpa_images: Vec<Handle<Image>> = config.images.iter()
        .map(|path| asset_server.load(path))
        .collect();
    
    // Fallback colors while images load
    let colors = [
        Color::srgb(1.0, 0.0, 0.0), // Red
        Color::srgb(0.0, 1.0, 0.0), // Green
        Color::srgb(0.0, 0.0, 1.0), // Blue
        Color::srgb(1.0, 1.0, 0.0), // Yellow
        Color::srgb(1.0, 0.0, 1.0), // Magenta
    ];
    
    #[cfg(target_arch = "wasm32")]
    console::log_1(&format!("Spawning {} dancing grandpas", config.num_dancers).into());
    
    for i in 0..config.num_dancers {
        let x = (i as f32 - 2.0) * 120.0; // Spread them out horizontally
        let y = 0.0;
        let base_position = Vec3::new(x, y, 0.0);
        
        // Use colored rectangles for now (images will be added later when they load)
        let color = colors[i % colors.len()];
        
        commands.spawn((
            Sprite {
                color,
                custom_size: Some(Vec2::new(100.0, 100.0)),
                ..default()
            },
            Transform::from_translation(base_position),
            DancingGrandpa {
                dancer_id: i,
                base_position,
                current_frame: 0,
                frame_timer: 0.0,
            },
        ));
    }
    
    // Load and play audio
    let audio_handle = asset_server.load(&config.audio_file);
    
    #[cfg(target_arch = "wasm32")]
    console::log_1(&"Loading audio file".into());
    
    // Start playing audio immediately
    commands.spawn((
        AudioPlayer::new(audio_handle.clone()),
        PlaybackSettings::LOOP,
    ));
    
    // Store minimal resources for animation
    commands.insert_resource(Config(config.clone()));
    commands.insert_resource(AnimationTimer(Timer::from_seconds(
        config.beat_duration() * 0.5,
        TimerMode::Repeating
    )));
    
    #[cfg(target_arch = "wasm32")]
    console::log_1(&"Setup system completed".into());
}

pub fn simple_animation_system(
    time: Res<Time>,
    mut animation_timer: ResMut<AnimationTimer>,
    mut dancers: Query<(&mut Transform, &DancingGrandpa)>,
) {
    // Tick the animation timer
    animation_timer.0.tick(time.delta());
    
    if animation_timer.0.just_finished() {
        #[cfg(target_arch = "wasm32")]
        console::log_1(&"Animation tick".into());
        
        // Simple bouncing animation
        for (mut transform, dancer) in dancers.iter_mut() {
            let bounce_height = 30.0;
            let time_factor = (time.elapsed_secs() + dancer.dancer_id as f32).sin();
            transform.translation.y = dancer.base_position.y + bounce_height * time_factor;
        }
    }
}

pub fn animation_system(
    time: Res<Time>,
    mut animation_timer: ResMut<AnimationTimer>,
    mut game_state: ResMut<GameState>,
    config: Res<Config>,
    loaded_images: Res<LoadedImages>,
    mut commands: Commands,
    mut dancers: Query<(Entity, &mut DancingGrandpa, &mut Transform, &mut Sprite, &ImageHandles)>,
    loaded_audio: Res<LoadedAudio>,
    mut audio_instance: ResMut<AudioInstance>,
    asset_server: Res<AssetServer>,
) {
    match *game_state {
        GameState::Loading => {
            #[cfg(target_arch = "wasm32")]
            console::log_1(&"In Loading state, spawning dancers immediately for testing...".into());
            
            // For testing, spawn dancers immediately without waiting for assets
            spawn_dancers(&mut commands, &config.0, &loaded_images.0);
            
            // Don't start audio for now, just focus on visual
            // let audio_entity = commands.spawn((
            //     AudioPlayer::new(loaded_audio.0.clone()),
            //     PlaybackSettings::ONCE,
            // )).id();
            // audio_instance.0 = Some(audio_entity);
            
            *game_state = GameState::Playing;
            #[cfg(target_arch = "wasm32")]
            console::log_1(&"Game state changed to Playing".into());
        }
        GameState::Playing => {
            if animation_timer.0.tick(time.delta()).just_finished() {
                update_dancer_animations(&mut dancers, &config.0);
            }
        }
        _ => {}
    }
}

pub fn audio_system(
    mut game_state: ResMut<GameState>,
    audio_instance: Res<AudioInstance>,
    audio_sinks: Query<&AudioSink>,
    mut fade_timer: ResMut<FadeTimer>,
) {
    if let GameState::Playing = *game_state {
        if let Some(entity) = audio_instance.0 {
            if let Ok(sink) = audio_sinks.get(entity) {
                if sink.empty() {
                    // Audio finished, start fading
                    *game_state = GameState::FadingOut;
                    fade_timer.0.reset();
                }
            }
        }
    }
}

pub fn fade_system(
    time: Res<Time>,
    mut fade_timer: ResMut<FadeTimer>,
    mut game_state: ResMut<GameState>,
    mut dancers: Query<(Entity, &mut Sprite), With<DancingGrandpa>>,
    mut commands: Commands,
    mut restart_timer: ResMut<RestartTimer>,
) {
    if let GameState::FadingOut = *game_state {
        if fade_timer.0.tick(time.delta()).just_finished() {
            // Fade completed, remove dancers and start silence
            for (entity, _) in dancers.iter() {
                commands.entity(entity).despawn();
            }
            *game_state = GameState::Silence;
            restart_timer.0.reset();
        } else {
            // Update fade
            let fade_progress = fade_timer.0.elapsed_secs() / fade_timer.0.duration().as_secs_f32();
            let alpha = 1.0 - fade_progress;
            
            for (_, mut sprite) in dancers.iter_mut() {
                sprite.color = sprite.color.with_alpha(alpha);
            }
        }
    }
}

pub fn restart_system(
    time: Res<Time>,
    mut restart_timer: ResMut<RestartTimer>,
    mut game_state: ResMut<GameState>,
    config: Res<Config>,
    loaded_images: Res<LoadedImages>,
    loaded_audio: Res<LoadedAudio>,
    mut commands: Commands,
    mut audio_instance: ResMut<AudioInstance>,
) {
    if let GameState::Silence = *game_state {
        if restart_timer.0.tick(time.delta()).just_finished() {
            // Restart the sequence
            spawn_dancers(&mut commands, &config.0, &loaded_images.0);
            
            // Start audio again
            let audio_entity = commands.spawn((
                AudioPlayer::new(loaded_audio.0.clone()),
                PlaybackSettings::ONCE,
            )).id();
            audio_instance.0 = Some(audio_entity);
            
            *game_state = GameState::Playing;
        }
    }
}

fn spawn_dancers(
    commands: &mut Commands,
    config: &DancingGrandpaConfig,
    images: &[Handle<Image>],
) {
    #[cfg(target_arch = "wasm32")]
    console::log_1(&format!("spawn_dancers called with {} dancers, {} images", config.num_dancers, images.len()).into());
    
    let mut rng = rand::thread_rng();
    let window_width = 800.0; // Assume window dimensions
    let window_height = 600.0;
    
    for i in 0..config.num_dancers {
        let x = rng.gen_range(-window_width/2.0..window_width/2.0);
        let y = rng.gen_range(-window_height/2.0..window_height/2.0);
        let base_position = Vec3::new(x, y, 0.0);
        
        #[cfg(target_arch = "wasm32")]
        console::log_1(&format!("Spawning dancer {} at position ({}, {})", i, x, y).into());
        
        // For now, spawn colored rectangles instead of images to test rendering
        let mut entity_commands = commands.spawn_empty();
        entity_commands.insert(Sprite {
            color: Color::srgb(0.0, 1.0, 0.0), // Green color
            custom_size: Some(Vec2::new(50.0, 50.0)),
            ..default()
        });
        entity_commands.insert(Transform::from_translation(base_position));
        entity_commands.insert(GlobalTransform::default());
        entity_commands.insert(Visibility::default());
        entity_commands.insert(InheritedVisibility::default());
        entity_commands.insert(ViewVisibility::default());
        entity_commands.insert(DancingGrandpa {
            dancer_id: i,
            base_position,
            current_frame: 0,
            frame_timer: 0.0,
        });
        entity_commands.insert(ImageHandles {
            handles: images.to_vec(),
        });
    }
}

fn update_dancer_animations(
    dancers: &mut Query<(Entity, &mut DancingGrandpa, &mut Transform, &mut Sprite, &ImageHandles)>,
    config: &DancingGrandpaConfig,
) {
    for (_, mut dancer, mut transform, mut sprite, image_handles) in dancers.iter_mut() {
        if config.animation_frames.is_empty() {
            continue;
        }
        
        // Move to next frame
        dancer.current_frame = (dancer.current_frame + 1) % config.animation_frames.len();
        let frame = &config.animation_frames[dancer.current_frame];
        
        // Update sprite image if we have the right image
        if frame.image_index < image_handles.handles.len() {
            sprite.image = image_handles.handles[frame.image_index].clone();
        }
        
        // Apply animation transformations
        transform.translation = dancer.base_position + Vec3::new(frame.offset_x, frame.offset_y, 0.0);
        transform.scale = Vec3::splat(frame.scale);
        transform.rotation = Quat::from_rotation_z(frame.rotation_degrees * PI / 180.0);
        
        // Update sprite color to ensure it's visible
        sprite.color = Color::WHITE;
    }
}