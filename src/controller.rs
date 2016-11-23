extern crate rustbox;

use rusoto::{
    DefaultCredentialsProvider,
    Region
};

use hyper::Client;
use self::rustbox::Key;
use super::kinesis::KinesisHelper;
use super::screen::Screen;


enum State {
    Root,
    StreamList(Vec<String>),
    ShardsList,    
}
    
pub struct Controller {
    screen: Screen
}

impl Controller {
    
    pub fn new() -> Controller {
                    
        Controller {
            screen: Screen::new()
        }
    }

    pub fn run(&self) {
        
        let credentials = DefaultCredentialsProvider::new().unwrap();
        let client = Client::new();        
        let kinesis_helper = KinesisHelper::new(client, credentials, Region::ApNortheast1);
        
        let mut state: State = State::Root;

        
        loop {
            
            state = match state {
                State::StreamList(streams) => {
                    self.screen.draw_strem_names(streams);
                    
                    match self.screen.poll_event() {
                        
                        Key::Char('q') => {
                            break;
                        },
                        _ => State::Root
                            
                    }
                },
                State::ShardsList => {
                    State::Root                    
                },
                State::Root => {
                    
                    self.screen.draw_help();

                    match self.screen.poll_event() {
                        
                        Key::Char('q') => {
                            break;
                        }
                        Key::Char('l') => {
                            match kinesis_helper.list_streams() {
                                Ok(streams) => {
                                    State::StreamList(streams)
                                }
                                Err(e) => {
                                    println!("{:?}", e);
                                    State::Root
                                }
                            }
                        },
                        _ => State::Root
                            
                    }                    
                    
                }
            };
                 
        }
    }
}

