extern crate rustbox;

use std::error::Error;
use self::rustbox::{Color, RustBox, Key};

pub struct Screen {
    rustbox: RustBox
}

impl Screen {
     
    pub fn new() -> Screen {
        let rustbox = match RustBox::init(Default::default()) {
            Result::Ok(v) => v,
            Result::Err(e) => panic!("{}", e),
        };

        Screen { rustbox: rustbox }
    }

    pub fn draw_help(&self) {
        self.rustbox.clear();                
        self.rustbox.print(0, 0, rustbox::RB_BOLD, Color::White, Color::Black, "> Hello!");
        self.rustbox.print(0, 1, rustbox::RB_BOLD, Color::White, Color::Black, "> Press 'q' to quit.");
        self.rustbox.print(0, 2, rustbox::RB_BOLD, Color::White, Color::Black, "> Press 'l' to show kinesis streams.");
        self.rustbox.present();        
    }    

    pub fn draw_strem_names(&self, stream_names: Vec<String>) {
        
        self.rustbox.clear();
        
        self.rustbox.print(0, 0, rustbox::RB_BOLD, Color::Green, Color::Black, "Kinesis Stream List");
        
        for (num, stream_name) in stream_names.iter().enumerate() {
            self.rustbox.print(0, num + 1, rustbox::RB_BOLD, Color::White, Color::Black, &format!("[{0}]: {1}", num, &stream_name));
        }

        self.rustbox.present();        
    }

    pub fn poll_event(&self) -> Key {
        match self.rustbox.poll_event(false) {
            Ok(rustbox::Event::KeyEvent(key)) => key,
            Err(e) => panic!("{}", e.description()),
            _ => panic!("error")
        }
    }
}

