extern crate rustbox;

use self::rustbox::{RustBox, Key};


use rusoto::{
    DefaultCredentialsProvider,
    Region
};

use rusoto::kinesis::Record;
use hyper::Client;
use super::kinesis::KinesisHelper;
use super::screen::Screen;
use super::screen::Status;

enum State {
    Root,
    End,
    StreamList(Vec<String>),
    ShardList(String, Vec<String>), 
    RecordList(String, Vec<Record>),  // stream_name, iterator_id
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

    pub fn run(&mut self) {
        
        let credentials = DefaultCredentialsProvider::new().unwrap();
        let client = Client::new();        
        let kinesis_helper = KinesisHelper::new(client, credentials, Region::ApNortheast1);
        
        let mut state: State = State::Root; 
        
        loop {
            
            state = match state {
                
                State::Root => {
                    
                    self.screen.draw_help();

                    match self.screen.poll_event() {
                        Key::Char('q') => State::End,
                        Key::Char('l') => {
                            match kinesis_helper.list_streams() {
                                Ok(streams) => State::StreamList(streams),
                                Err(e) =>  State::Root
                            }
                        },
                        _ => State::Root
                            
                    }                    
                    
                },
                State::StreamList(streams) => {

                    self.screen.update_lines(&streams);
                    
                    match self.screen.select_line() {
                        Status::Error | Status::Quit => State::End,
                        Status::Selected(i) => {
                            let ref stream_name = streams[i];
                            match kinesis_helper.describe_shards(stream_name) {
                                Ok(shards) => State::ShardList(stream_name.to_string(), shards),
                                Err(e) => State::Root,
                            } 
                        }
                        _ => State::Root
                    }
                },
                State::ShardList(stream_name, shards) => {
                    
                    self.screen.update_lines(&shards);

                    match self.screen.select_line() {
                        Status::Error | Status::Quit => State::End,
                        Status::Selected(i) => {
                            let ref shard_id = shards[i];
                            match kinesis_helper.get_shard_iterator(&stream_name, shard_id) {
                                Ok(shard_iterator) => State::RecordList(shard_iterator, vec!()),
                                Err(e) => State::Root,
                            }
                        }
                        _ => State::Root
                    }                    

                },
                State::RecordList(shard_iterator, records) => {

                    match kinesis_helper.get_records(&shard_iterator) {
                        Ok((r, i)) => {

                            if r.len() != 0 {
                                self.screen.draw_records(&kinesis_helper.decode_records(&r))
                            }
                            
                            match i {
                                Some(i) => {
                                    match self.screen.poll_event() {                                        
                                        Key::Char('n') => State::RecordList(i, r),
                                        Key::Char('q') => State::End,
                                        _ => State::Root
                                    }
                                },
                                None => {
                                    match self.screen.poll_event() {
                                        Key::Char('q') => State::End,
                                        _ => State::Root
                                    }
                                }
                            }
                        },
                        Err(e) => State::Root,
                    }
                    
                },
                State::End => {
                    break;
                }
            };
                 
        }
    }
}

