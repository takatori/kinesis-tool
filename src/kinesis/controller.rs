use hyper::Client;
use super::helper::KinesisHelper;
use utils::screen::Screen;
use utils::screen::ScreenStatus;
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


struct KinesisController {
    kinesis_helper: KinesisHelper<DefaultCredentialsProvider, Client>,
    screen: Screen,
}


impl KinesisController {

    pub fn new(credential_provider: DefaultCredentialsProvider,
               client: Client,
               region: Region) -> KinesisController {

        KinesisController {
            kinesis_helper: KinesisHelper::new(client, credential_provider, region),
            screen: Screen::new(),
        }
        
    }


    /// Event Loop
    pub fn run(&mut self) {

        let mut state = State::Root;
            
        loop {
            state = match state {
                State::Root                           => self.root(),
                State::StreamList(streams)            => self.stream_list(streams),
                State::ShardList(stream_name, shards) => self.shared_list(stream_name, shards),
                State::RecordList(shard_iterator)     => self.record_list(shard_iterator),
                State::Record(shard_iterator, record) => self.record(shard_iterator, record),
                State::End => {
                    break;
                }
            };
        }        
    }

    /// initial state
    fn root(&mut self) -> State {
        let commands = vec!["l".to_string(), "q".to_string()];
        self.screen.update_screen("🍓  Kinesis", &commands);

        match self.screen.select_line() {
            ScreenStatus::Error | ScreenStatus::Quit => State::End,
            ScreenStatus::Selected(ref c) if c == "l" => {
                match self.kinesis_helper.list_streams() {
                    Ok(streams) => State::StreamList(streams),
                    Err(e) =>  State::Root
                }
            }
            _ => State::Root
        }                        
    }

    fn stream_list(&mut self, streams: Vec<String>) -> State {
        
        self.screen.update_screen("🍓  Kinesis > Streams", &streams);

        match self.screen.select_line() {
            ScreenStatus::Error | ScreenStatus::Quit => State::End,
            ScreenStatus::Selected(stream_name) => {
                match self.kinesis_helper.describe_shards(&stream_name) {
                    Ok(shards) => State::ShardList(stream_name.to_string(), shards),
                    Err(e) => State::Root,
                } 
            }
            _ => State::Root
        }    
    }

    fn shared_list(&mut self, stream_name: String, shards: Vec<String>) -> State {
        
        self.screen.update_screen("🍓  Kinesis > Streams > Shards", &shards);

        match self.screen.select_line() {
            ScreenStatus::Error | ScreenStatus::Quit => State::End,
            ScreenStatus::Selected(shard_id) => {
                match self.kinesis_helper.get_shard_iterator(&stream_name, &shard_id) {
                    Ok(shard_iterator) => State::RecordList(shard_iterator),
                    Err(e) => State::Root,
                }
            }
            _ => State::Root
        }                    
        
    }

    fn record_list(&mut self, shard_iterator: String) -> State {

        match self.kinesis_helper.get_records(&shard_iterator) {
            
            Ok((results, iterator)) => {

                self.screen.update_screen("🍓  Kinesis > Streams > Shards > Records",
                                     &self.kinesis_helper.decode_records(&results));
                
                match self.screen.select_line() {
                    ScreenStatus::Error | ScreenStatus::Quit => State::End,
                    ScreenStatus::Selected(ref c) if c == "n" => {
                        match iterator {
                            Some(iterator) => State::RecordList(iterator),
                            None => State::Root
                        }
                    },
                    ScreenStatus::Selected(ref record) => State::Record(shard_iterator, record.to_string()),
                    _ => State::Root
                }
            },
            Err(e) => State::Root,
        }    
    }

    fn record(&mut self, shard_iterator: String, record: String) -> State {
        self.screen.update_screen("🍓  Kinesis > Streams > Shards > Records > Record", &vec!(String::new()));
        
        match self.screen.render(&self.kinesis_helper.format_record(&record)) {
            ScreenStatus::Error | ScreenStatus::Quit => State::End,
            ScreenStatus::Escaped  => State::RecordList(shard_iterator),
            _ => State::Root
        }                       
    }
    
}
