//! A helper to mix sound effects and music.
//!
//! It has 4 sound effect nodes that are automatically
//!  rotated when a new sound effect is played.
//!
//! # Examples
//!
//! ```
//! use ffz_utils::audio::*;
//! let mut audio_player = AudioPlayer::new(0.5, 1.0);
//! audio_player.play_music("theme_music");
//! audio_player.play_sfx("boom");
//! ```

use firefly_rust::audio::{self, Gain, Loop, Mix, Node};

pub struct AudioPlayer {
    #[allow(dead_code)]
    sfx_mix: Node<Mix>,
    sfx_nodes: [Node<Gain>; 4],
    sfx_index: usize,
    #[allow(dead_code)]
    music_gain: Node<Gain>,
    music_loop: Node<Loop>,
}

impl AudioPlayer {
    /// Create a new audio player
    ///
    /// - `music_gain` is a value `0.0..1.0` for the music volume.
    /// - `sfx_gain` is a value `0.0..1.0` for the sound effect volume.
    pub fn new(music_gain: f32, sfx_gain: f32) -> Self {
        let music_gain = audio::OUT.add_gain(music_gain);
        let music_loop = music_gain.add_loop();

        let sfx_mix = audio::OUT.add_mix();
        let sfx_nodes = [
            sfx_mix.add_gain(sfx_gain),
            sfx_mix.add_gain(sfx_gain),
            sfx_mix.add_gain(sfx_gain),
            sfx_mix.add_gain(sfx_gain),
        ];
        AudioPlayer {
            sfx_mix,
            sfx_nodes,
            sfx_index: 0,
            music_gain,
            music_loop,
        }
    }

    // Play a sound effect.
    //
    // A maximum of 4 sound effects an be played simultaneously.
    pub fn play_sfx(&mut self, file_name: &str) {
        self.sfx_nodes[self.sfx_index].clear();
        self.sfx_nodes[self.sfx_index].add_file(file_name);
        self.sfx_index = (self.sfx_index + 1) % 4;
    }

    /// Play the music file in a loop.
    ///
    /// Automatically removes any currently playing music.
    pub fn play_music(&mut self, file_name: &str) {
        self.stop_music();
        self.music_loop.add_file(file_name);
    }

    /// Stop the currently playing music.
    pub fn stop_music(&mut self) {
        self.music_loop.clear();
    }
}
