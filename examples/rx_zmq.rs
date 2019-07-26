extern crate byteorder;
extern crate libbladerf;
extern crate zmq;

use byteorder::{WriteBytesExt, LittleEndian};
use libbladerf::{BladeRF, Channel, Format, Layout};


fn main() -> std::io::Result<()> {
    let dev = BladeRF::open("").expect("Could not open device");
    let mut samples = [0i32; 2048];
    println!("Opened Serial: {}", dev.get_serial());

    dev.enable_module(Channel::RX_0, true)
        .expect("Could not enable RX channel");
    dev.set_sample_rate(Channel::RX_0, 2_500_000)
        .expect("Could not set sample rate");
    dev.set_bandwidth(Channel::RX_0, 2_500_000)
        .expect("Could not set bandwidth");
    dev.set_frequency(Channel::RX_0, 2_462_000_000)
        .expect("Could not set frequency");
    dev.sync_config(Layout::RX_X2, Format::SC16_Q11, 8, 1024, 4, 0)
        .expect("Could not config receive settings");

    // ZMQ Setup
    let ctx = zmq::Context::new();
    let rep_socket = ctx.socket(zmq::REP).unwrap();
    rep_socket.bind("tcp://*:5556").unwrap();

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

    Ok(())
}
