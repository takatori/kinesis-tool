extern crate hyper;
extern crate flate2;
extern crate serde_json;

pub mod top;
pub mod aws;
pub mod utils;

use top::controller::topController;

pub fn run() {
    let mut top = topController::new();
    top.run();    
}
