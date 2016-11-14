extern crate rusoto;
extern crate hyper;
    
mod controller;
mod kinesis;
mod screen;

use controller::Controller;

fn main() {
    
    let mut controller = Controller::new();
    controller.run();
    
}

