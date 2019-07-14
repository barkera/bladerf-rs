extern crate libbladerf;

use libbladerf::{BladeRF, Channel, Format, Layout};

fn main() {
    let dev = BladeRF::open("").expect("Could not open device");
    let mut samples = [0i32; 2048];
    println!("Opened Serial: {}", dev.get_serial());

    dev.enable_module(Channel::RX_0, true)
        .expect("Could not enable RX channel");
    dev.set_sample_rate(Channel::RX_0, 2_500_000)
        .expect("Could not set sample rate");
    dev.set_bandwidth(Channel::RX_0, 2_500_000)
        .expect("Could not set bandwidth");
    dev.set_frequency(Channel::RX_0, 88_700_000)
        .expect("Could not set frequency");
    dev.sync_config(Layout::RX_X1, Format::SC16_Q11, 8, 1024, 4, 0)
        .expect("Could not config receive settings");
    dev.sync_rx(&mut samples, 0)
        .expect("Could not receive samples");

    println!("Done.");
}
