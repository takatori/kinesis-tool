
use rusoto::{
    DefaultCredentialsProvider,
    ProvideAwsCredentials,
    DispatchSignedRequest,    
    Region
};
use hyper::Client;
use super::kinesis::KinesisHelper;
use super::screen::Screen;

pub struct Controller {
    kinesis_helper: KinesisHelper,
    screen: Screen
}

impl Controller {
    
    pub fn new() -> Controller {
        
        let credentials = DefaultCredentialsProvider::new().unwrap();
        let client = Client::new();

        Controller {
            kinesis_helper: KinesisHelper::new(client, credentials, Region::ApNortheast1),
            screen: Screen::new()
        }
    }

    pub fn run(&self) {
        // self.screen.draw_first();
        // self.screen.draw(&self.kinesis_helper);
    }
}

