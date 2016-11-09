extern crate rusoto;
extern crate rustbox;
extern crate hyper;

mod kinesis;
    
use std::error::Error;
use std::default::Default;

use rusoto::kinesis::{KinesisClient, ListStreamsInput};
use rusoto::{DefaultCredentialsProvider, ProvideAwsCredentials, DispatchSignedRequest, Region};

use rustbox::{Color, RustBox};
use rustbox::Key;

use kinesis::KinesisHelper;

use hyper::Client;

fn main() {
    
    let credentials    = DefaultCredentialsProvider::new().unwrap();
    let client = Client::new();    
    let kinesis_helper = KinesisHelper::new(client, credentials, Region::ApNortheast1);


    let rustbox = match RustBox::init(Default::default()) {
        Result::Ok(v) => v,
        Result::Err(e) => panic!("{}", e),
    };

    rustbox.print(1, 1, rustbox::RB_BOLD, Color::White, Color::Black, "Hello, world!");
    rustbox.print(1, 3, rustbox::RB_BOLD, Color::White, Color::Black, "Press 'q' to quit.");
    rustbox.present();

    loop {
        
        rustbox.clear();

        match rustbox.poll_event(false) {
            Ok(rustbox::Event::KeyEvent(key)) => {
                match key {
                    Key::Char('q') => { break; }
                    Key::Char('l') => { kinesis_helper.list_streams(); }
                    _ => { rustbox.clear(); }
                }
            },
            Err(e) => panic!("{}", e.description()),
            _ => { }
        }

        
    }
    
}
