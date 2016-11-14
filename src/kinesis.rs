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

pub struct KinesisHelper {
    client: KinesisClient<ProvideAwsCredentials, DispatchSignedRequest>
}

impl KinesisHelper {
    
    fn new(request_dispatcher: DispatchSignedRequest, credentials_provider: ProvideAwsCredentials, region: Region) -> KinesisHelper {
        
        KinesisHelper { client: KinesisClient::with_request_dispatcher(request_dispatcher, credentials_provider, region) }
    }
    
    fn list_streams(&self) -> Result<Vec<String>, Box<Error>> {
        
        let request = ListStreamsInput::default();
        let result = try!(self.client.list_streams(&request));
        Ok(result.stream_names)
            
    }

    fn describe_shards(&self, stream_name: String) -> Result<Vec<String>, Box<Error>> {

        let stream = DescribeStreamInput {
            stream_name: stream_name,
            limit: None,
            exclusive_start_shard_id: None,
        };
        
        let result = try!(self.client.describe_stream(&stream));
        Ok(result.stream_description.shards.iter().map(|x| &x.shard_id).cloned().collect())
            
    }
}

