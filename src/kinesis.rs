extern crate rusoto;

use std::default::Default;
use std::error::Error;
    
use rusoto::{
    ProvideAwsCredentials,
    DispatchSignedRequest,
    Region,
};
use rusoto::kinesis::{
    KinesisClient,
    ListStreamsInput,
    DescribeStreamInput,
};

pub struct KinesisHelper<P, D> where P: ProvideAwsCredentials, D: DispatchSignedRequest {
    client: KinesisClient<P, D>,
}

impl <P, D>KinesisHelper<P, D> where P: ProvideAwsCredentials, D: DispatchSignedRequest {
    
    pub fn new(request_dispatcher: D, credentials_provider: P, region: Region) -> KinesisHelper<P, D> {
        
        KinesisHelper { client: KinesisClient::with_request_dispatcher(request_dispatcher, credentials_provider, region) }
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
    
}
