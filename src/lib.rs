//! Loading and playing sounds.

#![allow(warnings)]

mod error;

pub use error::Error;

#[cfg(target_os = "android")]
#[path = "opensles_snd.rs"]
mod snd;

#[cfg(any(target_os = "linux", target_os = "dragonfly", target_os = "freebsd"))]
#[path = "alsa_snd.rs"]
mod snd;

#[cfg(any(target_os = "macos", target_os = "ios"))]
#[path = "coreaudio_snd.rs"]
mod snd;

#[cfg(target_os = "windows")]
#[path = "wasapi_snd.rs"]
mod snd;

#[cfg(target_arch = "wasm32")]
#[path = "web_snd.rs"]
mod snd;

#[cfg(not(target_arch = "wasm32"))]
mod mixer;

pub use snd::{AudioContext, Sound};

pub struct PlaySoundParams {
    pub looped: bool,
    pub volume: f32,
}

impl Default for PlaySoundParams {
    fn default() -> PlaySoundParams {
        PlaySoundParams {
            looped: false,
            volume: 1.,
        }
    }
}

#[derive(Copy, Clone)]
pub struct AudioParams {
    /// sample rate to use
    freq: usize,
    /// channel count
    channels: usize,
}
impl Default for AudioParams {
    fn default() -> Self {
        Self {
            freq: 44_800,
            channels: 1
        }
    }
}
pub trait AudioCallback {
    /// fills the audio buffer
    fn callback(&mut self, out: &mut [f32]) {

    }
}

pub struct AudioDevice<CB: AudioCallback> {
    spec: AudioParams,
    callback: Box<Option<CB>>
}

/// underlying access to the audio system
/// provided on a platform based layer
pub struct AudioSystem;
impl AudioSystem {
    /// loads the given Callback & Params setup
    fn open_device<CB: AudioCallback, F>(spec: AudioParams, func: F) -> AudioDevice<CB>
    where F: FnOnce(AudioParams) -> CB {
        AudioDevice {
            spec: spec.clone(),
            callback: Box::new(Some((func)(spec)))
        }
    }
}
pub type AudioCallbackSetup<'a> = FnOnce(AudioParams) -> &'a dyn AudioCallback;

/// trait used to generate audio
pub trait AudioDeviceImpl {
    /// attempts to start audio playback
    fn resume(&mut self) -> Result<(), Error>;
}
