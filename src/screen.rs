extern crate rustbox;
extern crate regex;

use std::error::Error;
use self::rustbox::{Color, RustBox, Key};
use self::rustbox::Event::KeyEvent;
use self::regex::Regex;


// Error 

// Screen Status
pub enum Status {
    Selected(String),
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
    lines: Vec<String>, 
    prompt: String,
    y_offset: usize,
    filtered: Vec<String>,
    query: String,
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
            filtered: vec!(),
            prompt: "> ".to_owned(),
            y_offset: 1,
            query: String::new(),
            cursor: 0,
            // offset: 0,
            rustbox: rustbox
        }
    }

    pub fn update_screen(&mut self, lines: &Vec<String>) {
        self.cursor = 0;
        self.query = String::new();
        self.lines = lines.to_owned();
        self.filtered = self.lines.clone();
    }


    pub fn select_line(&mut self) -> Status {
        
        loop {
            
            self.render_items();
            
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
                Ok(Status::Selected(self.filtered[self.cursor].to_owned()))
            },
            KeyEvent(Key::Esc) => Ok(Status::Escaped),
            KeyEvent(Key::Up) | KeyEvent(Key::Ctrl('p')) => {
                self.cursor_up();
                Ok(Status::Continue)
            },
            KeyEvent(Key::Down) | KeyEvent(Key::Ctrl('n')) => {
                self.cursor_down();
                Ok(Status::Continue)
            },
            KeyEvent(Key::Char('q')) => {
                Ok(Status::Quit)
            },            
            KeyEvent(Key::Backspace) => self.remove_query().and(Ok(Status::Continue)),
            KeyEvent(Key::Char(c)) => self.append_query(c).and(Ok(Status::Continue)),
            _ => Ok(Status::Continue),            
        }
    }

    fn append_query(&mut self, c: char) -> Result<(), Box<Error>> {
        
        self.query.push(c);
        self.apply_filter()
    }

    fn remove_query(&mut self) -> Result<(), Box<Error>> {
        
        if self.query.is_empty() {
            return Ok(());
        }

        let idx = self.query.len() - 1;
        self.query.remove(idx);
        self.apply_filter()
    }

    fn apply_filter(&mut self) -> Result<(), Box<Error>> {
        self.filtered = if self.query.len() == 0 {
            self.lines.clone()
        } else {
            let re = try!(Regex::new(self.query.as_str()));
            self.lines.iter().filter(|&input| re.is_match(input)).cloned().collect()
        };

        self.cursor = 0;

        Ok(())
    }    

    fn cursor_up(&mut self) {
        
        if self.cursor != 0 { self.cursor -= 1; }
    }

    fn cursor_down(&mut self) {
        
        self.cursor += 1;  // displayのサイズを考慮する必要がある
    }

    
    fn render_items(&self) {
        
        self.rustbox.clear();
        
        for(y, item) in self.filtered.iter().enumerate() {
            if y == self.cursor {
                self.rustbox.print_line(y + self.y_offset, &format!("[{0}]: {1}", y, &item), Color::Black, Color::Green);
            } else {
                self.rustbox.print_line(y + self.y_offset, &format!("[{0}]: {1}", y, &item), Color::Blue, Color::Black);
            }
        }

        // print query line and move the cursor to end.
        let query_str = format!("{}{}", self.prompt, self.query);
        self.rustbox.print_line(0, &query_str, Color::Green, Color::Black);
        self.rustbox.set_cursor(query_str.len() as isize, 0);
        self.rustbox.present();                
    }

    
    pub fn draw_records(&self, records: &Vec<String>) {
        
        self.rustbox.clear();
        
        self.rustbox.print_line(1, "☰ Kinesis > Streams > Shards > Records", Color::Black, Color::Green);
        self.rustbox.print_line(2, "☰ Press 'n' to next page.", Color::Black, Color::Green);                        

        for (num, record) in records.iter().enumerate() {
            self.rustbox.print_line(num + 3, &format!("[{0}]: {1}", num, &record), Color::Blue, Color::Black);                                            
        }        

        self.rustbox.present();                        
    }

    
}

