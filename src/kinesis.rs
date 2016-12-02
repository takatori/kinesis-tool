extern crate rusoto;
extern crate flate2;

use std::default::Default;
use std::error::Error;

use std::io::prelude::*;
use self::flate2::read::GzDecoder;
    
use rusoto::{
    ProvideAwsCredentials,
    DispatchSignedRequest,
    Region,
};

use rusoto::kinesis::{
    KinesisClient,
    ListStreamsInput,
    DescribeStreamInput,
    GetShardIteratorInput,
    GetRecordsInput,
    Record,
};



pub struct KinesisHelper<P, D> where P: ProvideAwsCredentials, D: DispatchSignedRequest {
    client: KinesisClient<P, D>,
}

impl <P, D>KinesisHelper<P, D> where P: ProvideAwsCredentials, D: DispatchSignedRequest {
    
    pub fn new(request_dispatcher: D, credentials_provider: P, region: Region) -> KinesisHelper<P, D> {
        
        KinesisHelper { client: KinesisClient::with_request_dispatcher(request_dispatcher, credentials_provider, region) }
    }

    pub fn decode_records(&self, records: &[Record]) -> Vec<String> {
        records.iter().map(|record| self.decode(&record)).collect::<Vec<String>>()
    }


    pub fn decode(&self, record: &Record) -> String {
        
        let mut d = GzDecoder::new(&record.data[..]).unwrap();
        let mut s = String::new();
        d.read_to_string(&mut s).unwrap();
        s
    }

    
    pub fn list_streams(&self) -> Result<Vec<String>, Box<Error>> {
        
        let request = ListStreamsInput::default();
        let result = try!(self.client.list_streams(&request));
        Ok(result.stream_names)
            
    }

    pub fn describe_shards(&self, stream_name: &str) -> Result<Vec<String>, Box<Error>> {

        let stream = DescribeStreamInput {
            stream_name: stream_name.to_string(),
            limit: None,
            exclusive_start_shard_id: None,
        };
        
        let result = try!(self.client.describe_stream(&stream));
        Ok(result.stream_description.shards.iter().map(|x| &x.shard_id).cloned().collect())
            
    }

    pub fn get_shard_iterator(&self, stream_name: &str, shard_id: &str) -> Result<String, Box<Error>> {

        let input = GetShardIteratorInput {
            shard_id: shard_id.to_string(),
            starting_sequence_number: None,            
            shard_iterator_type: "TRIM_HORIZON".to_string(),
            stream_name: stream_name.to_string(),
            timestamp: None,            
        };
        
        let result = try!(self.client.get_shard_iterator(&input));
        Ok(result.shard_iterator.unwrap())
    }

    pub fn get_records(&self, shard_iterator: &str) -> Result<(Vec<Record>, Option<String>), Box<Error>> {

        let input = GetRecordsInput {
            // limit: Option<GetRecordsInputLimit>,
            limit: None,
            shard_iterator: shard_iterator.to_string(),
        };

        let result = try!(self.client.get_records(&input));
        Ok((result.records, result.next_shard_iterator))
    }

}
