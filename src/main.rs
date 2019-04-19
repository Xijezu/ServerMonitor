extern crate ini;
extern crate mysql;
extern crate serde;

mod database;
mod monitor;
mod rc4;

fn main() {
    let mut mon = monitor::Monitor::default();
    mon.init();
    mon.start();
}
