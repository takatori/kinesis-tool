extern crate rusoto;
extern crate hyper;
extern crate flate2;

mod kinesis;
mod utils;

use hyper::Client;
use rusoto::{
    DefaultCredentialsProvider,
    Region
};

use kinesis::controller;

pub fn run() {
    let credential_provider = DefaultCredentialsProvider::new().unwrap();
    let client              = Client::new();        
    controller::run(credential_provider, client);
}
