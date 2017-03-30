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
    RecordList(String),
    Record(String, String),
}

trait Controller {
    fn run(&self, credential_provider: DefaultCredentialsProvider, client: Client, region: Region) -> ()
}


struct KinesisController {
    kinesis_helper: KinesisHelper,
    screen: Screen,
}


impl KinesisController {

    pub fn new() -> KinesisController {
        
    }

    fn root() -> State {
        
    }
}


pub fn run(credential_provider: DefaultCredentialsProvider, client: Client, region: Region) {
    
    let kinesis_helper = KinesisHelper::new(client, credential_provider, region);
    let mut screen: Screen = Screen::new();
    let mut state: State = State::Root; 
    
    loop {
        state = match state {
            State::Root                           => root(&screen, &kinesis_helper),
            State::StreamList(streams)            => stream_list(&screen, &kinesis_helper, streams),
            State::ShardList(stream_name, shards) => shared_list(&screen, &kinesis_helper, stream_name, shards),
            State::RecordList(shard_iterator)     => record_list(&screen, &kinesis_helper, shard_iterator),
            State::Record(shard_iterator, record) => record(&screen, &kinesis_helper, shard_iterator, record),
            State::End => {
                break;
            }
        };
    }
}


fn root(screen: &mut Screen, kinesis_helper: &KinesisHelper) -> State {

    let commands = vec!["l".to_string(), "q".to_string()];
    screen.update_screen("🍓  Kinesis", &commands);

    match screen.select_line() {
        Status::Error | Status::Quit => State::End,
        Status::Selected(ref c) if c == "l" => {
            match kinesis_helper.list_streams() {
                Ok(streams) => State::StreamList(streams),
                Err(e) =>  State::Root
            }
        }
        _ => State::Root
    }                        
}


fn stream_list(screen: &mut Screen, kinesis_helper: &KinesisHelper, streams: Vec<String>) -> State {
    
    screen.update_screen("🍓  Kinesis > Streams", &streams);

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
}

fn shared_list(screen: &mut Screen, kinesis_helper: &KinesisHelper, stream_name: &str, shards: Vec<String>) -> State {
    
    screen.update_screen("🍓  Kinesis > Streams > Shards", &shards);

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
    
}

fn record_list(screen: &mut Screen, kinesis_helper: &KinesisHelper, shard_iterator: String) -> State {

    match kinesis_helper.get_records(&shard_iterator) {
        
        Ok((results, iterator)) => {

            screen.update_screen("🍓  Kinesis > Streams > Shards > Records",
                                 &kinesis_helper.decode_records(&results));
            
            match screen.select_line() {
                Status::Error | Status::Quit => State::End,
                Status::Selected(ref c) if c == "n" => {
                    match iterator {
                        Some(iterator) => State::RecordList(iterator),
                        None => State::Root
                    }
                },
                Status::Selected(ref record) => State::Record(&shard_iterator, record.to_string()),
                _ => State::Root
            }
        },
        Err(e) => State::Root,
    }    
}

fn record(screen: &mut Screen, kinesis_helper: &KinesisHelper, shard_iterator: String, record: String) -> State {
    screen.update_screen("🍓  Kinesis > Streams > Shards > Records > Record", &vec!(String::new()));
    
    match screen.render(&kinesis_helper.format_record(&record)) {
        Status::Error | Status::Quit => State::End,
        Status::Escaped  => State::RecordList(shard_iterator),
        _ => State::Root
    }                       
}
