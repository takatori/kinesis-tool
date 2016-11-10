extern crate rusoto;

use std::default::Default;

use rusoto::{
    ProvideAwsCredentials,
    DispatchSignedRequest,    
    Region
};
use rusoto::kinesis::{
    KinesisClient,
    ListStreamsInput
};


pub struct KinesisHelper<P, D> where P: ProvideAwsCredentials, D: DispatchSignedRequest {
    client: KinesisClient<P, D>
}

impl<P: ProvideAwsCredentials, D: DispatchSignedRequest> KinesisHelper<P, D> {
    
    pub fn new(request_dispatcher: D, credentials_provider: P, region: Region) -> KinesisHelper<P, D> {
        
        KinesisHelper { client: KinesisClient::with_request_dispatcher(request_dispatcher, credentials_provider, region) }
    }
    
    pub fn list_streams(&self) -> Vec<String> {
        
        let request = ListStreamsInput::default();

        match self.client.list_streams(&request) {
            Ok(output) => {
                output.stream_names
            }
            Err(error) => {
                println!("Error: {:?}", error);
                vec!()
            }
        }
    }
}

