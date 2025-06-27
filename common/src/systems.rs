use bevy::prelude::*;
use bevy::audio::{PlaybackSettings, AudioPlayer};
use rand::Rng;
use std::f32::consts::PI;

use crate::{
    components::*,
    resources::*,
    config::DancingGrandpaConfig,
};

pub fn setup_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    // Load default config
    let config = DancingGrandpaConfig::default();
    
    // Load images
    let images: Vec<Handle<Image>> = config.images.iter()
        .map(|path| asset_server.load(path))
        .collect();
    
    // Load audio
    let audio = asset_server.load(&config.audio_file);
    
    // Setup camera
    commands.spawn(Camera2d);
    
    // Store resources
    commands.insert_resource(Config(config.clone()));
    commands.insert_resource(LoadedImages(images));
    commands.insert_resource(LoadedAudio(audio));
    commands.insert_resource(AudioInstance(None));
    
    // Update animation timer based on tempo
    let beat_duration = config.beat_duration();
    commands.insert_resource(AnimationTimer(Timer::from_seconds(
        beat_duration * 0.5, // Half beat for smoother animation
        TimerMode::Repeating
    )));
    
    // Update fade timer
    commands.insert_resource(FadeTimer(Timer::from_seconds(
        config.fade_duration_secs,
        TimerMode::Once
    )));
    
    // Update restart timer
    commands.insert_resource(RestartTimer(Timer::from_seconds(
        config.silence_duration_secs,
        TimerMode::Once
    )));
}

pub fn animation_system(
    time: Res<Time>,
    mut animation_timer: ResMut<AnimationTimer>,
    mut game_state: ResMut<GameState>,
    config: Res<Config>,
    loaded_images: Res<LoadedImages>,
    mut commands: Commands,
    mut dancers: Query<(Entity, &mut DancingGrandpa, &mut Transform, &mut Sprite)>,
    loaded_audio: Res<LoadedAudio>,
    mut audio_instance: ResMut<AudioInstance>,
) {
    match *game_state {
        GameState::Loading => {
            // Check if assets are loaded
            if !loaded_images.0.is_empty() {
                spawn_dancers(&mut commands, &config.0, &loaded_images.0);
                
                // Start audio
                let audio_entity = commands.spawn((
                    AudioPlayer::new(loaded_audio.0.clone()),
                    PlaybackSettings::ONCE,
                )).id();
                audio_instance.0 = Some(audio_entity);
                
                *game_state = GameState::Playing;
            }
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
    let mut rng = rand::thread_rng();
    let window_width = 800.0; // Assume window dimensions
    let window_height = 600.0;
    
    for i in 0..config.num_dancers {
        let x = rng.gen_range(-window_width/2.0..window_width/2.0);
        let y = rng.gen_range(-window_height/2.0..window_height/2.0);
        let base_position = Vec3::new(x, y, 0.0);
        
        let initial_image = if !images.is_empty() {
            images[0].clone()
        } else {
            continue;
        };
        
        let mut entity_commands = commands.spawn_empty();
        entity_commands.insert(Sprite::default());
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
            handles: vec![initial_image],
        });
    }
}

fn update_dancer_animations(
    dancers: &mut Query<(Entity, &mut DancingGrandpa, &mut Transform, &mut Sprite)>,
    config: &DancingGrandpaConfig,
) {
    for (_, mut dancer, mut transform, mut sprite) in dancers.iter_mut() {
        if config.animation_frames.is_empty() {
            continue;
        }
        
        // Move to next frame
        dancer.current_frame = (dancer.current_frame + 1) % config.animation_frames.len();
        let frame = &config.animation_frames[dancer.current_frame];
        
        // Apply animation transformations
        transform.translation = dancer.base_position + Vec3::new(frame.offset_x, frame.offset_y, 0.0);
        transform.scale = Vec3::splat(frame.scale);
        transform.rotation = Quat::from_rotation_z(frame.rotation_degrees * PI / 180.0);
        
        // Update sprite color to ensure it's visible
        sprite.color = Color::WHITE;
    }
}