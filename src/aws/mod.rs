extern crate rusoto;

mod kinesis;
// mod ec2;

use std::process;
use self::kinesis::controller::KinesisController;
use self::rusoto::{DefaultCredentialsProvider, Region};

use ::hyper::Client;
use ::utils::screen::Screen;
use ::utils::screen::ScreenStatus;


pub struct awsController {
    screen: Screen,    
}

impl awsController {
    
    pub fn new() -> awsController {
        awsController {
            screen: Screen::new(),
        }
    }

    pub fn run(&mut self) {
        
        let commands = vec!["kinesis".to_string(), "q".to_string()];
        self.screen.update_screen(r###"
  ________  ___       __   ________      
 |\   __  \|\  \     |\  \|\   ____\     
 \ \  \|\  \ \  \    \ \  \ \  \___|_    
  \ \   __  \ \  \  __\ \  \ \_____  \   
   \ \  \ \  \ \  \|\__\_\  \|____|\  \  
    \ \__\ \__\ \____________\____\_\  \ 
     \|__|\|__|\|____________|\_________\
                             \|_________|

   select aws service   
   "###, &commands);

        match self.screen.select_line() {           
            ScreenStatus::Selected(ref c) if c == "0" => {
                
            },
            _ => process::exit(0),            
        }        
    }

    fn kinesis(&self) {
        let credential_provider = DefaultCredentialsProvider::new().unwrap();
        let client              = Client::new();        
        let mut controlelr = KinesisController::new(credential_provider, client, Region::ApNortheast1);
        controlelr.run();
    }
    
}

