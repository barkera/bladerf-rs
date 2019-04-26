#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

use std::ffi::{CStr, CString};

pub mod error;

use error::Error;

pub type Result<T> = std::result::Result<T, Error>;

pub struct BladeRF {
    dev: *mut bladerf,
    devinfo: bladerf_devinfo,
}

pub enum Channel {
    RX_0 = BLADERF_CHANNEL_RX_0 as isize,
    RX_1 = BLADERF_CHANNEL_RX_1 as isize,
    TX_0 = BLADERF_CHANNEL_TX_0 as isize,
    TX_1 = BLADERF_CHANNEL_TX_1 as isize,
}

pub enum GainMode {
    DEFAULT = bladerf_gain_mode_BLADERF_GAIN_DEFAULT as isize,
    MGC = bladerf_gain_mode_BLADERF_GAIN_MGC as isize,
    FAST_AGC = bladerf_gain_mode_BLADERF_GAIN_FASTATTACK_AGC as isize,
    SLOW_AGC = bladerf_gain_mode_BLADERF_GAIN_SLOWATTACK_AGC as isize,
    HYBRID_AGC = bladerf_gain_mode_BLADERF_GAIN_HYBRID_AGC as isize,
}

impl GainMode {
    fn from_u32(mode: u32) -> Option<GainMode> {
        match mode {
            0 => Some(GainMode::DEFAULT),
            1 => Some(GainMode::MGC),
            2 => Some(GainMode::FAST_AGC),
            3 => Some(GainMode::SLOW_AGC),
            4 => Some(GainMode::HYBRID_AGC),
            _ => None,
        }
    }
}

pub enum Backend {
    Any = bladerf_backend_BLADERF_BACKEND_ANY as isize,
    Linux = bladerf_backend_BLADERF_BACKEND_LINUX as isize,
    LibUSB = bladerf_backend_BLADERF_BACKEND_LIBUSB as isize,
    Cypress = bladerf_backend_BLADERF_BACKEND_CYPRESS as isize,
    Dummy = bladerf_backend_BLADERF_BACKEND_DUMMY as isize,
    Unknown,
}

impl BladeRF {
    pub fn open(ident_str: &str) -> Result<Self> {
        let (dev, devinfo) = unsafe {
            let ident_str = CString::new(ident_str)?;

            let mut dev: *mut bladerf = std::ptr::null_mut();
            let mut devinfo: bladerf_devinfo = std::mem::zeroed();

            let rc = bladerf_open(&mut dev, ident_str.as_ptr());
            if rc < 0 {
                return Err(Error::from(rc));
            }

            let rc = bladerf_get_devinfo(dev, &mut devinfo);
            if rc < 0 {
                return Err(Error::from(rc));
            }

            (dev, devinfo)
        };

        Ok(BladeRF { dev, devinfo })
    }

    /// Gain values in dB, can be positive or negative
    pub fn set_gain(&self, channel: Channel, gain: i32) -> Result<()> {
        unsafe {
            let rc = bladerf_set_gain(
                self.dev,
                channel as bladerf_channel,
                gain,
            );
            if rc < 0 {
                return Err(Error::from(rc));
            }
        };

        Ok(())
    }

    pub fn get_gain(&self, channel: Channel) -> Result<i32> {
        let gain = unsafe {
            let mut gain: i32 = 0;
            let rc = bladerf_get_gain(
                self.dev,
                channel as bladerf_channel,
                &mut gain,
            );
            if rc < 0 {
                return Err(Error::from(rc));
            }

            gain
        };

        Ok(gain)
    }

    pub fn set_gain_mode(&self, channel: Channel, gain_mode: GainMode) -> Result<()> {
        unsafe {
            let rc = bladerf_set_gain_mode(
                self.dev,
                channel as bladerf_channel,
                gain_mode as bladerf_gain_mode,
            );
            if rc < 0 {
                return Err(Error::from(rc));
            }
        }

        Ok(())
    }

    pub fn get_gain_mode(&self, channel: Channel) -> Result<GainMode> {
        let mode = unsafe {
            let mut mode: bladerf_gain_mode = 0;
            let rc = bladerf_get_gain_mode(
                self.dev,
                channel as bladerf_channel,
                &mut mode,
            );
            if rc < 0 {
                return Err(Error::from(rc));
            }

            if let Some(i) = GainMode::from_u32(mode) {
                i
            } else {
                return Err(Error::Unexpected);
            }
        };

        Ok(mode)
    }

    pub fn set_sample_rate(&self, channel: Channel, rate: u32) -> Result<u32> {
        let actual = unsafe {
            let mut actual: u32 = 0;
            let rc = bladerf_set_sample_rate(
                self.dev,
                channel as bladerf_channel,
                rate,
                &mut actual,
            );
            if rc < 0 {
                return Err(Error::from(rc));
            }

            actual
        };

        Ok(actual)
    }

    pub fn get_sample_rate(&self, channel: Channel) -> Result<u32> {
        let actual = unsafe {
            let mut actual: u32 = 0;
            let rc = bladerf_get_sample_rate(
                self.dev,
                channel as bladerf_channel,
                &mut actual,
            );
            if rc < 0 {
                return Err(Error::from(rc));
            }

            actual
        };

        Ok(actual)
    }

    pub fn set_bandwidth(
        &self,
        channel: Channel,
        bandwidth: u32,
    ) -> Result<u32> {
        let actual = unsafe {
            let mut actual: u32 = 0;
            let rc = bladerf_set_bandwidth(
                self.dev,
                channel as bladerf_channel,
                bandwidth,
                &mut actual,
            );
            if rc < 0 {
                return Err(Error::from(rc));
            }

            actual
        };

        Ok(actual)
    }

    pub fn get_bandwidth(&self, channel: Channel) -> Result<u32> {
        let actual = unsafe {
            let mut actual: u32 = 0;
            let rc = bladerf_get_bandwidth(
                self.dev,
                channel as bladerf_channel,
                &mut actual,
            );
            if rc < 0 {
                return Err(Error::from(rc));
            }

            actual
        };

        Ok(actual)
    }

    pub fn set_frequency(&self, channel: Channel, freq: u64) -> Result<()> {
        unsafe {
            let rc = bladerf_set_frequency(
                self.dev,
                channel as bladerf_channel,
                freq as bladerf_frequency,
            );
            if rc < 0 {
                return Err(Error::from(rc));
            }

            Ok(())
        }
    }

    pub fn get_frequency(&self, channel: Channel) -> Result<u64> {
        let freq = unsafe {
            let mut freq: u64 = 0;
            let rc = bladerf_get_frequency(
                self.dev,
                channel as bladerf_channel,
                &mut freq,
            );
            if rc < 0 {
                return Err(Error::from(rc));
            }

            freq
        };

        Ok(freq)
    }

    pub fn get_serial(&self) -> String {
        let s = unsafe { CStr::from_ptr(self.devinfo.serial.as_ptr()) };

        if let Ok(s) = s.to_str() {
            s.to_string()
        } else {
            String::from("<Invalid string>")
        }
    }

    pub fn get_product(&self) -> String {
        let s = unsafe { CStr::from_ptr(self.devinfo.product.as_ptr()) };

        if let Ok(s) = s.to_str() {
            s.to_string()
        } else {
            String::from("<Invalid string>")
        }
    }

    pub fn get_manufacturer(&self) -> String {
        let s = unsafe { CStr::from_ptr(self.devinfo.manufacturer.as_ptr()) };

        if let Ok(s) = s.to_str() {
            s.to_string()
        } else {
            String::from("<Invalid string>")
        }
    }

    pub fn get_backend(&self) -> Backend {
        Backend::from(self.devinfo.backend)
    }
}

impl Drop for BladeRF {
    fn drop(&mut self) {
        unsafe {
            bladerf_close(self.dev);
        }
    }
}

impl std::fmt::Display for BladeRF {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            fmt,
            "Description: {} {}\n\
             Backend:     {}\n\
             Serial:      {}\n\
             USB Bus:     {}\n\
             USB Address: {}",
            self.get_manufacturer(),
            self.get_product(),
            self.get_backend(),
            self.get_serial(),
            self.devinfo.usb_bus,
            self.devinfo.usb_addr
        )
    }
}

impl From<u32> for Backend {
    fn from(num: u32) -> Backend {
        match num {
            bladerf_backend_BLADERF_BACKEND_ANY => Backend::Any,
            bladerf_backend_BLADERF_BACKEND_LINUX => Backend::Linux,
            bladerf_backend_BLADERF_BACKEND_LIBUSB => Backend::LibUSB,
            bladerf_backend_BLADERF_BACKEND_CYPRESS => Backend::Cypress,
            bladerf_backend_BLADERF_BACKEND_DUMMY => Backend::Dummy,
            _ => Backend::Unknown,
        }
    }
}

impl std::fmt::Display for Backend {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        let string = match self {
            Backend::Any => "any",
            Backend::Linux => "linux",
            Backend::LibUSB => "libusb",
            Backend::Cypress => "cypress API",
            Backend::Dummy => "dummy",
            Backend::Unknown => "unknown",
        };

        write!(fmt, "{}", string)
    }
}
