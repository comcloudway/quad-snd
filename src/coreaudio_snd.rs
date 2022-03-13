use crate::{
    error::Error,
    AudioDevice,
    AudioDeviceImpl,
    AudioCallback,
    AudioParams
};

use std::sync::mpsc;

#[path = "coreaudio/coreaudio.rs"]
mod coreaudio;

// bindgen lost some defines from coreaudio.rs
const _saudio_kAudioFormatLinearPCM: u32 = 1819304813; //'lpcm';
const _saudio_kLinearPCMFormatFlagIsFloat: u32 = 1 << 0;
const _saudio_kAudioFormatFlagIsPacked: u32 = 1 << 3;

use coreaudio::*;

mod consts {
    pub const DEVICE: &'static str = "default\0";
    pub const BUFFER_FRAMES: u32 = 4096;
}
unsafe extern "C" fn saudio_coreaudio_callback(
    user_data: *mut ::std::os::raw::c_void,
    queue: _saudio_AudioQueueRef,
    buffer: _saudio_AudioQueueBufferRef,
) {
    let mut cb = &mut *(user_data as *mut AudioCallback);

    let num_frames = (*buffer).mAudioDataByteSize / (2 * 4);
    let buf =
        std::slice::from_raw_parts_mut((*buffer).mAudioData as *mut f32, num_frames as usize * 2);

    cb.callback(&mut buf, num_frames as usize);

    AudioQueueEnqueueBuffer(queue, buffer, 0, std::ptr::null_mut());
}

impl<CB> AudioDeviceImpl for AudioDevice<CB> where CB: AudioCallback {
    fn resume(&mut self) -> Result<(), String> {
        if let Some(cb) = self.callback.take() {
            let cb = Box::new(cb);

            unsafe {
                let fmt = _saudio_AudioStreamBasicDescription {
                    mSampleRate: self.spec.freq as f64,
                    mFormatID: _saudio_kAudioFormatLinearPCM,
                    mFormatFlags: _saudio_kLinearPCMFormatFlagIsFloat
                        | _saudio_kAudioFormatFlagIsPacked,
                    mFramesPerPacket: 1,
                    mChannelsPerFrame: self.spec.channels as u32,
                    mBytesPerFrame: 4 * self.spec.channels as u32,
                    mBytesPerPacket: 4 * self.spec.channels as u32,
                    mBitsPerChannel: 32,
                    mReserved: 0,
                };
                let mut ca_audio_queue: _saudio_AudioQueueRef = std::mem::zeroed();
                let res = AudioQueueNewOutput(
                    &fmt,
                    Some(saudio_coreaudio_callback),
                    Box::into_raw(cb) as *mut _,
                    std::ptr::null_mut(),
                    std::ptr::null_mut(),
                    0,
                    &mut ca_audio_queue,
                );
                assert!(res == 0);
                assert!(ca_audio_queue.is_null() == false);

                // create 2 audio buffers
                for _ in 0..2 {
                    let mut buf: _saudio_AudioQueueBufferRef = std::ptr::null_mut();
                    let buf_byte_size = consts::BUFFER_FRAMES * fmt.mBytesPerFrame;
                    let res = AudioQueueAllocateBuffer(ca_audio_queue, buf_byte_size, &mut buf);
                    assert!(res == 0);
                    assert!(buf.is_null() == false);
                    (*buf).mAudioDataByteSize = buf_byte_size;
                    std::ptr::write_bytes(
                        (*buf).mAudioData as *mut u8,
                        0,
                        (*buf).mAudioDataByteSize as usize,
                    );
                    AudioQueueEnqueueBuffer(ca_audio_queue, buf, 0, std::ptr::null_mut());
                }

                let res = AudioQueueStart(ca_audio_queue, std::ptr::null_mut());
                assert!(res == 0);
            }
            Ok(())
        } else {
            Err(String::from("No AudioCallback found"))
        }
    }
}
