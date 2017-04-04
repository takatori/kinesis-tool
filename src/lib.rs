extern crate rusoto;
extern crate hyper;
extern crate flate2;
extern crate serde_json;

mod kinesis;
mod utils;

use hyper::Client;
use rusoto::{DefaultCredentialsProvider, Region};

use kinesis::controller;

pub fn run() {
    top();
    // let credential_provider = DefaultCredentialsProvider::new().unwrap();
    // let client              = Client::new();        
    // controller::run(credential_provider, client, Region::ApNortheast1);
}


pub fn top() {

    println!(r###"
-----------------------------------------

██████╗  █████╗ ██╗    ██╗███████╗████████╗
██╔══██╗██╔══██╗██║    ██║██╔════╝╚══██╔══╝
██████╔╝███████║██║ █╗ ██║███████╗   ██║   
██╔══██╗██╔══██║██║███╗██║╚════██║   ██║   
██║  ██║██║  ██║╚███╔███╔╝███████║   ██║   
╚═╝  ╚═╝╚═╝  ╚═╝ ╚══╝╚══╝ ╚══════╝   ╚═╝   

------------------------------------------                                           

rawst is the interactive tool for AWS CLI written in Rust.

1) AWS
2) GCP

"###);
}
