#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

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

impl BladeRF {
    pub fn new() -> Result<Self> {
        let (dev, devinfo) = unsafe {
            let mut dev: *mut bladerf = std::ptr::null_mut();
            let mut devinfo: bladerf_devinfo = std::mem::zeroed();

            bladerf_init_devinfo(&mut devinfo);

            let rc = bladerf_open_with_devinfo(&mut dev, &mut devinfo);
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
        self.devinfo
            .serial
            .to_vec()
            .into_iter()
            .map(|c| c as u8 as char)
            .collect::<String>()
    }
}

impl Drop for BladeRF {
    fn drop(&mut self) {
        unsafe {
            bladerf_close(self.dev);
        }
    }
}
