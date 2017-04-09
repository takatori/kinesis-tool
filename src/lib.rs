pub mod top;
pub mod aws;
mod utils;

use top::controller::topController;

pub fn run() {
    let mut top = topController::new();
    top.run();    
}
