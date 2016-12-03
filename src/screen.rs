extern crate rustbox;

use std::error::Error;
use self::rustbox::{Color, RustBox, Key};

trait PrintLine {
    fn print_line(&self, y: usize, item: &str, fg: Color, bg: Color);
}

impl PrintLine for RustBox {
    
    fn print_line(&self, y: usize, item: &str, fg: Color, bg: Color) {
        for x in 0..(self.width()) {
            let ch = item.chars().nth(x).unwrap_or(' ');
            self.print_char(x, y, rustbox::RB_NORMAL, fg, bg, ch);
        }
    }
}



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
        self.rustbox.print_line(0, "☰ Hello! this is kinesis helper tool.", Color::Black, Color::Green);
        self.rustbox.print_line(1, "☰ Press 'l' to show kinesis streams.", Color::Blue, Color::Black);                
        self.rustbox.print_line(2, "☰ Press 'q' to quit.", Color::Blue, Color::Black);
        self.rustbox.present();        
    }    

    pub fn draw_strem_names(&self, stream_names: &Vec<String>) {
        
        self.rustbox.clear();

        self.rustbox.print_line(0, "☰ Kinesis > Streams", Color::Black, Color::Green);        
        
        for (num, stream_name) in stream_names.iter().enumerate() {
            self.rustbox.print_line(num + 1, &format!("☰ [{0}]: {1}", num, &stream_name), Color::Blue, Color::Black);                    
        }

        self.rustbox.present();        
    }

    pub fn draw_shards(&self, shards: &Vec<String>) {

        self.rustbox.clear();

        self.rustbox.print_line(0, "☰ Kinesis > Streams > Shards", Color::Black, Color::Green);                

        for (num, shards) in shards.iter().enumerate() {
            self.rustbox.print_line(num + 1, &format!("☰ [{0}]: {1}", num, &shards), Color::Blue, Color::Black);                                
        }        

        self.rustbox.present();                
    }

    pub fn draw_records(&self, records: &Vec<String>) {
        
        self.rustbox.clear();
        
        self.rustbox.print_line(0, "☰ Kinesis > Streams > Shards > Records", Color::Black, Color::Green);                

        for (num, record) in records.iter().enumerate() {
            self.rustbox.print_line(num + 1, &format!("☰ [{0}]: {1}", num, &record), Color::Blue, Color::Black);                                            
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

