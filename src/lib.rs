#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[derive(Debug)]
pub enum Error {
    InvalidDevice,
}

pub type Result<T> = std::result::Result<T, Error>;

pub struct BladeRf {
    dev: *mut bladerf,
    devinfo: bladerf_devinfo,
}

impl BladeRf {
    pub fn new() -> Result<Self> {
        let (dev, devinfo) = unsafe {
            let mut dev: *mut bladerf = std::ptr::null_mut();
            let mut devinfo: bladerf_devinfo = std::mem::zeroed();

            bladerf_init_devinfo(&mut devinfo);

            if bladerf_open_with_devinfo(&mut dev, &mut devinfo) != 0 {
                return Err(Error::InvalidDevice);
            }

            if bladerf_get_devinfo(dev, &mut devinfo) != 0 {
                return Err(Error::InvalidDevice);
            }

            (dev, devinfo)
        };

        Ok(BladeRf { dev, devinfo })
    }

    pub fn set_frequency_rx0(&self, freq: u64) -> Result<()> {
        unsafe {
            if bladerf_set_frequency(self.dev, BLADERF_CHANNEL_RX_0, freq as bladerf_frequency) != 0 {
                return Err(Error::InvalidDevice);
            }

            Ok(())
        }
    }

    pub fn get_frequency_rx0(&self) -> Result<u64> {
        let freq = unsafe {
            let mut freq: u64 = 0;
            if bladerf_get_frequency(self.dev, BLADERF_CHANNEL_RX_0, &mut freq) != 0 {
                return Err(Error::InvalidDevice);
            }

            freq
        };

        Ok(freq)
    }

    pub fn print_info(&self) {
        println!("{}", self.devinfo.serial.to_vec().into_iter().map(|c| c as u8 as char).collect::<String>());
    }
}

impl Drop for BladeRf {
    fn drop(&mut self) {
        unsafe {
            bladerf_close(self.dev);
        }
    }
}
