use quad_snd::{
    AudioCallback,
    AudioSystem,
    AudioParams,
    AudioDeviceImpl
};

static SAMPLE_RATE: usize = 44_800;

struct Sine {
    phase: f32,
    freq: f32,
    sample_rate: usize
}
impl AudioCallback for Sine {
    fn callback(&mut self, buffer: &mut [f32], _frames: usize) {
        for x in buffer {
            *x = f32::sin(2.*std::f32::consts::PI*self.freq*self.phase);
            self.phase+=1./(self.sample_rate as f32);
        }
    }
}

pub fn main() {
    let specs = AudioParams {
        freq: SAMPLE_RATE,
        channels: 1
    };

    let mut device = AudioSystem::open_device(specs, |specs| {
        Sine {
            freq: 440.,
            phase: 0.,
            sample_rate: specs.freq
        }
    });

    device.resume().expect("Failed to open audio device");
}
