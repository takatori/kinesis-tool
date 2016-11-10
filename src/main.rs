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

pub struct Screen {
    rustbox: RustBox
}

impl Screen {
    
    pub fn new() -> Screen {
        let rustbox = match RustBox::init(Default::default()) {
            Result::Ok(v) => v,
            Result::Err(e) => panic!("{}", e),
        };

        Screen { rustbox: rustbox }
    }

    
    pub fn draw_first(&self) {
        self.rustbox.print(0, 0, rustbox::RB_BOLD, Color::White, Color::Black, "> Hello, world!");
        self.rustbox.print(0, 1, rustbox::RB_BOLD, Color::White, Color::Black, "> Press 'q' to quit.");
        self.rustbox.print(0, 2, rustbox::RB_BOLD, Color::White, Color::Black, "> Press 'l' to show kinesis streams.");    
        self.rustbox.present();   
    }

    pub fn draw<P, D>(&self, kinesis_helper: &KinesisHelper<P, D>) where P: ProvideAwsCredentials, D: DispatchSignedRequest {
        
        loop {
            
            self.rustbox.clear();

            match self.rustbox.poll_event(false) {
                Ok(rustbox::Event::KeyEvent(key)) => {
                    match key {
                        Key::Char('q') => {
                            break;
                        },
                        Key::Char('l') => {
                            let streams = kinesis_helper.list_streams();
                            self.rustbox.print(0, 1, rustbox::RB_BOLD, Color::White, Color::Black, &streams);
                        },
                        _ => { }
                    }
                },
                Err(e) => panic!("{}", e.description()),
                _ => { }
            }
            self.rustbox.present();
        }
    }
}



fn main() {
    
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client = Client::new();    
    let kinesis_helper = KinesisHelper::new(client, credentials, Region::ApNortheast1);
    let screen = Screen::new();
    screen.draw(&kinesis_helper);
}

