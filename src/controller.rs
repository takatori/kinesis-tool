extern crate rustbox;

use self::rustbox::Key;


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
    RecordList(String),  // iterator_id, records
    Record(String, String), // itrator_id, record
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

                    let commands = vec!["list streams".to_string(), "quit".to_string()];
                    self.screen.update_screen(&commands);

                    match self.screen.select_line() {
                        Status::Error | Status::Quit => State::End,
                        Status::Selected(ref c) if c == "list streams" => {
                            match kinesis_helper.list_streams() {
                                Ok(streams) => State::StreamList(streams),
                                Err(e) =>  State::Root
                            }
                        }
                        _ => State::Root
                    }                    
                },
                State::StreamList(streams) => {

                    self.screen.update_screen(&streams);

                    match self.screen.select_line() {
                        Status::Error | Status::Quit => State::End,
                        Status::Selected(stream_name) => {
                            match kinesis_helper.describe_shards(&stream_name) {
                                Ok(shards) => State::ShardList(stream_name.to_string(), shards),
                                Err(e) => State::Root,
                            } 
                        }
                        _ => State::Root
                    }
                },
                State::ShardList(stream_name, shards) => {
                    
                    self.screen.update_screen(&shards);

                    match self.screen.select_line() {
                        Status::Error | Status::Quit => State::End,
                        Status::Selected(shard_id) => {
                            match kinesis_helper.get_shard_iterator(&stream_name, &shard_id) {
                                Ok(shard_iterator) => State::RecordList(shard_iterator),
                                Err(e) => State::Root,
                            }
                        }
                        _ => State::Root
                    }                    

                },
                State::RecordList(shard_iterator) => {

                    match kinesis_helper.get_records(&shard_iterator) {
                        
                        Ok((results, iterator)) => {

                            self.screen.update_screen(&kinesis_helper.decode_records(&results));
                            
                            match self.screen.select_line() {
                                Status::Error | Status::Quit => State::End,
                                Status::Selected(ref c) if c == "n" => {
                                    match iterator {
                                        Some(iterator) => State::RecordList(iterator),
                                        None => State::Root
                                    }
                                },
                                Status::Selected(ref record) => State::Record(shard_iterator, record.to_string()),
                                _ => State::Root
                            }
                        },
                        Err(e) => State::Root,
                    }
                },
                State::Record(shard_iterator, record) => {

                    self.screen.update_screen(&vec![record]);

                    match self.screen.select_line() {
                        Status::Error | Status::Quit => State::End,
                        Status::Selected(ref c) if c == "b"  => State::RecordList(shard_iterator),
                        _ => State::Root
                    }                                                            
                }
                State::End => {
                    break;
                }
            };
                 
        }
    }
}

