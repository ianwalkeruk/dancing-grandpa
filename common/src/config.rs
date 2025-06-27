use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DancingGrandpaConfig {
    pub images: Vec<String>,
    pub audio_file: String,
    pub tempo_bpm: f32,
    pub animation_frames: Vec<AnimationFrame>,
    pub num_dancers: usize,
    pub fade_duration_secs: f32,
    pub silence_duration_secs: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimationFrame {
    pub image_index: usize,
    pub duration_beats: f32,
    pub scale: f32,
    pub rotation_degrees: f32,
    pub offset_x: f32,
    pub offset_y: f32,
}

impl Default for DancingGrandpaConfig {
    fn default() -> Self {
        Self {
            images: vec![
                "grandpa1.png".to_string(),
                "grandpa2.png".to_string(),
                "grandpa3.png".to_string(),
            ],
            audio_file: "dance_music.wav".to_string(),
            tempo_bpm: 120.0,
            animation_frames: vec![
                AnimationFrame {
                    image_index: 0,
                    duration_beats: 0.5,
                    scale: 1.0,
                    rotation_degrees: 0.0,
                    offset_x: 0.0,
                    offset_y: 0.0,
                },
                AnimationFrame {
                    image_index: 1,
                    duration_beats: 0.5,
                    scale: 1.1,
                    rotation_degrees: 10.0,
                    offset_x: 5.0,
                    offset_y: -5.0,
                },
                AnimationFrame {
                    image_index: 2,
                    duration_beats: 0.5,
                    scale: 0.9,
                    rotation_degrees: -10.0,
                    offset_x: -5.0,
                    offset_y: 5.0,
                },
                AnimationFrame {
                    image_index: 1,
                    duration_beats: 0.5,
                    scale: 1.1,
                    rotation_degrees: -10.0,
                    offset_x: -5.0,
                    offset_y: -5.0,
                },
            ],
            num_dancers: 5,
            fade_duration_secs: 3.0,
            silence_duration_secs: 2.0,
        }
    }
}

impl DancingGrandpaConfig {
    pub fn load_from_file(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let content = std::fs::read_to_string(path)?;
        let config: Self = serde_json::from_str(&content)?;
        Ok(config)
    }

    pub fn save_to_file(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let content = serde_json::to_string_pretty(self)?;
        std::fs::write(path, content)?;
        Ok(())
    }

    pub fn beat_duration(&self) -> f32 {
        60.0 / self.tempo_bpm
    }
}