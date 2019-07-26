extern crate byteorder;
extern crate config;
extern crate libbladerf;
extern crate zmq;


use byteorder::{WriteBytesExt, LittleEndian};
use std::collections::HashMap;
use libbladerf::{BladeRF, Channel, Format, Layout};


fn main() {

    // Pull in settings from Config.toml
    let mut settings = config::Config::default();
    settings.merge(config::File::with_name("Config")).unwrap();
    let settings_map = settings.try_into::<HashMap<String, String>>().unwrap();

    let srate = settings_map["sample_rate"].parse::<u32>()
        .expect("Invalid setting in Config.toml for sample_rate");
    let bandwidth = settings_map["bandwidth"].parse::<u32>()
        .expect("Invalid setting in Config.toml for bandwidth");
    let rffreq = settings_map["rf_frequency"].parse::<u64>()
        .expect("Invalid setting in Config.toml for rffreq");
    let port = settings_map["port"].parse::<u16>()
        .expect("Invalid setting in Config.toml for port");

    // Set up the bladerf appropriately
    let dev = BladeRF::open("").expect("Could not open device");
    let mut samples = [0i32; 2048];
    println!("Opened Serial: {}", dev.get_serial());

    dev.enable_module(Channel::RX_0, true)
        .expect("Could not enable RX channel");
    dev.set_sample_rate(Channel::RX_0, srate)
        .expect("Could not set sample rate");
    dev.set_bandwidth(Channel::RX_0, bandwidth)
        .expect("Could not set bandwidth");
    dev.set_frequency(Channel::RX_0, rffreq)
        .expect("Could not set frequency");
    dev.sync_config(Layout::RX_X2, Format::SC16_Q11, 8, 1024, 4, 0)
        .expect("Could not config receive settings");

    // ZMQ Setup
    let ctx = zmq::Context::new();
    let rep_socket = ctx.socket(zmq::REP).unwrap();
    rep_socket.bind(&format!("tcp://*:{}", port)).unwrap();

    let mut dummy_msg = zmq::Message::new();
    loop {
        // Await a REQ
        rep_socket.recv(&mut dummy_msg, 0).unwrap();

        // Capture samples
        dev.sync_rx(&mut samples, 0)
            .expect("Could not receive samples");
        let mut msg = Vec::new();
        for sam in &samples.to_vec() {
            msg.write_i32::<LittleEndian>(*sam).unwrap();
        }

        // Send a REP
        rep_socket.send(msg, 0).unwrap();
    }
}
