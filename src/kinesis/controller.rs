use hyper::Client;
use super::helper::KinesisHelper;
use utils::screen::Screen;
use utils::screen::Status;

use rusoto::{
    DefaultCredentialsProvider,
    Region
};

enum State {
    Root,
    End,
    StreamList(Vec<String>),
    ShardList(String, Vec<String>), 
    RecordList(String),  // iterator_id, records
    Record(String, String), // itrator_id, record
}


pub fn run(credential_provider: DefaultCredentialsProvider, client: Client) {
    
    let kinesis_helper = KinesisHelper::new(client, credential_provider, Region::ApNortheast1);
    let mut screen: Screen = Screen::new();
    let mut state: State = State::Root; 
    
    loop {
        
        state = match state {
            
            State::Root => {

                let commands = vec!["list streams".to_string(), "quit".to_string()];
                screen.update_screen(&commands);

                match screen.select_line() {
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

                screen.update_screen(&streams);

                match screen.select_line() {
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
                
                screen.update_screen(&shards);

                match screen.select_line() {
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

                        screen.update_screen(&kinesis_helper.decode_records(&results));
                        
                        match screen.select_line() {
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

                match screen.render(&record) {
                    Status::Error | Status::Quit => State::End,
                    Status::Escaped  => State::RecordList(shard_iterator),
                    _ => State::Root
                }                                                            
            }
            State::End => {
                break;
            }
        };
        
    }
}
