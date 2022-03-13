use quad_snd::{
    AudioCallback,
    AudioSystem,
    AudioParams,
    AudioDeviceImpl
};

static SAMPLE_RATE: usize = 44_800;

struct Sine {
    phase: f32,
    freq: f32
}
impl AudioCallback for Sine {
    fn callback(&mut self, buffer: &mut [f32], frames: usize) {
        for x in buffer {
            *x = f32::sin(2.*std::f32::consts::PI*self.freq*self.phase);
            self.phase+=1./(SAMPLE_RATE as f32);
        }
    }
}

pub fn main() {
    let specs = AudioParams::default();

    let mut device = AudioSystem::open_device(specs, |specs| {
        Sine {
            freq: 440.,
            phase: 0.
        }
    });

    device.resume();
}
