extern crate rustbox;

use std::error::Error;
use self::rustbox::{Color, RustBox, Key};
use self::rustbox::Event::KeyEvent;

// Error処理

// Screen Status
pub enum Status {
    Selected(usize),
    Escaped,
    Continue,
    Quit,
    Error,
}

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
    lines: Vec<String>, // フィールドレベルのミュータビリティ    
    // prompt: String,
    // y_offset: usize,
    // filtered: Vec<String>,
    // query: String,
    cursor: usize,
    // offset: usize,
    rustbox: RustBox
}

impl Screen {
     
    pub fn new() -> Screen {
        
        let rustbox = match RustBox::init(Default::default()) {
            Result::Ok(v) => v,
            Result::Err(e) => panic!("{}", e),
        };

        Screen {
            lines: vec!(),
            // prompt: "☰ ".to_owned(),
            // y_offset: 1,
            cursor: 0,
            // offset: 0,
            rustbox: rustbox
        }
    }


    pub fn select_line(&mut self) -> Status {
        
        loop {
            
            match self.rustbox.poll_event(false) {
                
                Err(err) => panic!("{:?}", err),
                Ok(event) => match self.handle_event(event) {
                    Ok(Status::Selected(s)) => return Status::Selected(s),
                    Ok(Status::Quit) => return Status::Quit,                    
                        _ => (),
                    }
            }
        }
    }

    
    pub fn poll_event(&self) -> Key {
        match self.rustbox.poll_event(false) {
            Ok(rustbox::Event::KeyEvent(key)) => key,
            Err(e) => panic!("{}", e.description()),
            _ => panic!("error")
        }
    }    

    

    fn handle_event(&mut self, event: rustbox::Event) -> Result<Status, Box<Error>> {
        
        match event {
            KeyEvent(Key::Enter) => {
                Ok(Status::Selected(self.cursor))
            },
            KeyEvent(Key::Esc) => Ok(Status::Escaped),
            KeyEvent(Key::Up) => {
                self.cursor_up();
                Ok(Status::Continue)
            },
            KeyEvent(Key::Down) => {
                self.cursor_down();
                Ok(Status::Continue)
            },
            KeyEvent(Key::Char('q')) => {
                Ok(Status::Quit)
            },            
            // KeyEvent(Key::Backspace) => self.remove_query().and(Ok(Continue)),
            // KeyEvent(Key::Char(c)) => self.append_query(c).and(Ok(Continue)),
            _ => Ok(Status::Continue),            
        }
    }

    fn cursor_up(&mut self) {
        
        if self.cursor != 0 { self.cursor -= 1; }
    }

    fn cursor_down(&mut self) {
        
        self.cursor += 1;  // displayのサイズを考慮する必要がある
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
            self.rustbox.print_line(num + 1, &format!("[{0}]: {1}", num, &stream_name), Color::Blue, Color::Black);                    
        }

        self.rustbox.present();        
    }

    pub fn draw_shards(&self, shards: &Vec<String>) {

        self.rustbox.clear();

        self.rustbox.print_line(0, "☰ Kinesis > Streams > Shards", Color::Black, Color::Green);                

        for (num, shards) in shards.iter().enumerate() {
            self.rustbox.print_line(num + 1, &format!("[{0}]: {1}", num, &shards), Color::Blue, Color::Black);                                
        }        

        self.rustbox.present();                
    }

    pub fn draw_records(&self, records: &Vec<String>) {
        
        self.rustbox.clear();
        
        self.rustbox.print_line(0, "☰ Kinesis > Streams > Shards > Records", Color::Black, Color::Green);
        self.rustbox.print_line(1, "☰ Press 'n' to next page.", Color::Black, Color::Green);                        

        for (num, record) in records.iter().enumerate() {
            self.rustbox.print_line(num + 2, &format!("[{0}]: {1}", num, &record), Color::Blue, Color::Black);                                            
        }        

        self.rustbox.present();                        
    }

    
}

