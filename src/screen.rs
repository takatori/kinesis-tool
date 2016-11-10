extern crate rustbox;

use std::error::Error;
use self::rustbox::{Color, RustBox};
use self::rustbox::Key;
use super::rusoto::{ProvideAwsCredentials, DispatchSignedRequest};
use kinesis::KinesisHelper;

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

    pub fn draw_strem_names(&self, streamNames: Vec<String>) {

        for (num, streamName) in streamNames.iter().enumerate() {
            self.rustbox.print(0, num, rustbox::RB_BOLD, Color::Green, Color::Black, &streamName);
        }
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
                            self.draw_strem_names(streams)
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
