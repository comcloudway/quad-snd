use quad_snd::{
    AudioCallback,
    AudioSystem,
    AudioParams,
    AudioDeviceImpl
};

/// store the sample rate globally
static SAMPLE_RATE: usize = 44_100;

/// a simple sine wave generator
struct Sine {
    phase: f32,
    freq: f32,
    sample_rate: usize
}
impl AudioCallback for Sine {
    fn callback(&mut self, buffer: &mut [f32], _frames: usize) {
        let mut val = 0.0;
        for (i,x) in buffer.iter_mut().enumerate() {
            if i%2==0 {
                val = f32::sin(2.*std::f32::consts::PI*self.freq*self.phase);
                self.phase+=1./(self.sample_rate as f32);
            }
            *x = val;
        }
    }
}

/// the main function of our app
pub fn main() {
    // specify some audio params
    let specs = AudioParams {
        // project sample rate
        freq: SAMPLE_RATE,
        // mono channel output
        // NOTE: Multichannel setups have to be handled manually
        channels: 2
    };

    // create a new audio device
    // and pass the AudioParams, so the generator struct has access
    let mut device = AudioSystem::open_device(specs, |specs| {
        Sine {
            freq: 440.,
            phase: 0.,
            sample_rate: specs.freq
        }
    });

    // spawn a second thread
    // keeps the main thread from locking
    std::thread::spawn(move || {
        // starts the audio thread
        device.resume().expect("Failed to open audio device");
    });

    // keep the main thread running
    std::io::stdin().read_line(&mut String::new())
                    .expect("Failed to keep thread running");
}
