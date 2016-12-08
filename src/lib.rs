extern crate rusoto;
extern crate hyper;
extern crate flate2;

pub mod kinesis;
pub mod utils;

use kinesis::controller::Controller;

pub fn run() {
    let mut controller = Controller::new();
    controller.run();
}
