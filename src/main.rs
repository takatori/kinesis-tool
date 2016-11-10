extern crate rusoto;
extern crate hyper;

mod kinesis;
mod screen;
    
use rusoto::{DefaultCredentialsProvider, Region};
use hyper::Client;

use kinesis::KinesisHelper;
use screen::Screen;

fn main() {
    
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client = Client::new();    
    let kinesis_helper = KinesisHelper::new(client, credentials, Region::ApNortheast1);
    let screen = Screen::new();
    screen.draw_first();
    screen.draw(&kinesis_helper);
}

