extern crate ini;
extern crate serde;

mod monitor;
mod rc4;

fn main() {
    let mut mon = monitor::Monitor::default();
    mon.init();
    mon.start();
}
