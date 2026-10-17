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
    pub fn new(gain: f32, sfx_gain: f32) -> Self {
        let music_gain = audio::OUT.add_gain(gain);
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

    pub fn play_sfx(&mut self, file_name: &str) {
        self.sfx_nodes[self.sfx_index].clear();
        self.sfx_nodes[self.sfx_index].add_file(file_name);
        self.sfx_index = (self.sfx_index + 1) % 4;
    }

    pub fn play_music(&mut self, file_name: &str) {
        self.music_loop.clear();
        self.music_loop.add_file(file_name);
    }
}
