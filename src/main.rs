extern crate rusoto;

use std::default::Default;

use rusoto::kinesis::{KinesisClient, ListStreamsInput};
use rusoto::{DefaultCredentialsProvider, Region};

fn main() {
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client = KinesisClient::new(credentials, Region::ApNortheast1);
    let request = ListStreamsInput::default();

    match client.list_streams(&request) {
        Ok(output) => {
            println!("{:?}", output);
        }
        Err(error) => {
            println!("Error: {:?}", error);
        }
    };
}
