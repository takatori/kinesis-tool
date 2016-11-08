extern crate rusoto;
extern crate rustbox;

use std::error::Error;
use std::default::Default;

use rusoto::kinesis::{KinesisClient, ListStreamsInput};
use rusoto::{DefaultCredentialsProvider, ProvideAwsCredentials, DispatchSignedRequest, Region};


use rustbox::{Color, RustBox};
use rustbox::Key;


fn list_streams<P: ProvideAwsCredentials, D: DispatchSignedRequest>(client: &KinesisClient<P, D>) {

    let request = ListStreamsInput::default();

    match client.list_streams(&request) {
        Ok(output) => {
            println!("{:?}", output);
        }
        Err(error) => {
            println!("Error: {:?}", error);
        }
    };
}


fn main() {
    
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client = KinesisClient::new(credentials, Region::ApNortheast1);

    let rustbox = match RustBox::init(Default::default()) {
        Result::Ok(v) => v,
        Result::Err(e) => panic!("{}", e),
    };

    rustbox.print(1, 1, rustbox::RB_BOLD, Color::White, Color::Black, "Hello, world!");
    rustbox.print(1, 3, rustbox::RB_BOLD, Color::White, Color::Black, "Press 'q' to quit.");
    rustbox.present();

    loop {
        match rustbox.poll_event(false) {
            Ok(rustbox::Event::KeyEvent(key)) => {
                match key {
                    Key::Char('q') => { break; }
                    Key::Char('l') => { list_streams(&client); }
                    _ => { }
                }
            },
            Err(e) => panic!("{}", e.description()),
            _ => { }
        }
    }
    
}
