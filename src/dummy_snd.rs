use crate::{
    AudioCallback,
    AudioParams,
    AudioDeviceImpl,
    AudioDevice
};

impl AudioDeviceImpl for AudioDevice {
    fn resume(&self) -> Result<(), String> {
        Ok(())
    }
}
