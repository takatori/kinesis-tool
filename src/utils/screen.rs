extern crate rustbox;
extern crate regex;

use std::cmp::min;
use std::error::Error;
use self::rustbox::{Color, RustBox, Key};
use self::rustbox::Event::KeyEvent;
use self::regex::Regex;


// Screen Status
pub enum ScreenStatus {
    Selected(String),
    Escaped,
    Continue,
    Quit,
    Error,
}

// Screen parts
pub struct Screen {
    pub header:   String,
    pub lines:    Vec<String>, 
    prompt:   String,
    // offset: usize,    
    y_offset: usize,
    filtered: Vec<String>,
    query:    String,
    cursor:   usize,
    rustbox:  RustBox
}

impl Screen {
    
    pub fn new() -> Screen {
        
        let rustbox = match RustBox::init(Default::default()) {
            Result::Ok(v) => v,
            Result::Err(e) => panic!("{}", e),
        };

        Screen {
            header: String::new(),
            lines: vec!(),
            filtered: vec!(),
            prompt: "💠   ".to_owned(),
            y_offset: 1,
            query: String::new(),
            cursor: 0,
            // offset: 0,
            rustbox: rustbox
        }
    }

    pub fn update_screen(&mut self, header: &str, lines: &Vec<String>) {
        self.y_offset = header.lines().count() + 1;
        self.header   = header.to_string();
        self.cursor   = 0;
        self.query    = String::new();
        self.lines    = lines.to_owned();
        self.filtered = self.lines.clone();
    }

    pub fn render(&self, item: &str) -> ScreenStatus {
        
        self.rustbox.clear();
        self.rustbox.print_lines(0, &self.header, Color::Green, Color::Black);
        self.rustbox.print_lines(self.header.lines().count(), item, Color::Blue, Color::Black);        
        self.rustbox.present();

        match self.rustbox.poll_event(false) {
            Ok(KeyEvent(Key::Char('q'))) => ScreenStatus::Quit,
            _ => ScreenStatus::Escaped
        }
    }
    

    pub fn select_line(&mut self) -> ScreenStatus {
        
        loop {
            
            self.render_items();
            
            match self.rustbox.poll_event(false) {
                Err(err) => panic!("{:?}", err),
                Ok(event) => match self.handle_event(event) {
                    Ok(ScreenStatus::Selected(s)) => return ScreenStatus::Selected(s),
                    Ok(ScreenStatus::Quit) => return ScreenStatus::Quit,                    
                    _ => (),
                }
            }
        }
    }
    

    fn handle_event(&mut self, event: rustbox::Event) -> Result<ScreenStatus, Box<Error>> {
        
        match event {
            KeyEvent(Key::Enter)     => Ok(ScreenStatus::Selected(self.filtered[self.cursor].to_owned())),
            KeyEvent(Key::Esc)       => Ok(ScreenStatus::Escaped),
            KeyEvent(Key::Char('q')) => Ok(ScreenStatus::Quit),
            KeyEvent(Key::Char(c))   => self.append_query(c).and(Ok(ScreenStatus::Continue)),            
            KeyEvent(Key::Backspace) => self.remove_query().and(Ok(ScreenStatus::Continue)),
            KeyEvent(Key::Up) |
            KeyEvent(Key::Ctrl('p')) => {
                self.cursor_up();
                Ok(ScreenStatus::Continue)
            },
            KeyEvent(Key::Down) |
            KeyEvent(Key::Ctrl('n')) => {
                self.cursor_down();
                Ok(ScreenStatus::Continue)
            },
            _ => Ok(ScreenStatus::Continue),            
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
        
        let height = self.rustbox.height();
        
        self.cursor = min(
            (self.cursor + 1),
            min(
                (self.filtered.len() - 1),
                (height - 1)
            )
        );
    }

    
    fn render_items(&self) {
        
        self.rustbox.clear();
        
        for(y, item) in self.filtered.iter().take(&self.rustbox.height() - 1 - self.y_offset).enumerate() {
            if y == self.cursor {
                self.rustbox.print_line(y + self.y_offset, &format!("[{0}]: {1}", y, &item), Color::Black, Color::Green);
            } else {
                self.rustbox.print_line(y + self.y_offset, &format!("[{0}]: {1}", y, &item), Color::Blue, Color::Black);
            }
        }

        // print query line and move the cursor to end.
        let query_str = format!("{}{}", self.prompt, self.query);
        self.rustbox.print_lines(0, &self.header, Color::Green, Color::Black);
        self.rustbox.print_line(self.y_offset - 1, &query_str, Color::Blue, Color::Black);
        self.rustbox.present();                
    }
    
}


trait Print {
    fn print_line(&self, y: usize, item: &str, fg: Color, bg: Color);
    fn print_lines(&self, start: usize, item: &str, fg: Color, bg: Color);
}


impl Print for RustBox {
    
    fn print_line(&self, y: usize, item: &str, fg: Color, bg: Color) {
        for x in 0..(self.width()) {
            let ch = item.chars().nth(x).unwrap_or(' ');
            self.print_char(x, y, rustbox::RB_NORMAL, fg, bg, ch);
        }
    }

    
    fn print_lines(&self, start: usize, item: &str, fg: Color, bg: Color) {

        let mut cursor = start;
        
        for line in item.split("\n") {
            
            // fold string if line lingth longer than window widht
            if line.chars().count() > self.width() {
                
                let (first, last) = line.split_at(self.width());
                self.print_line(cursor,    first, fg, bg);
                self.print_line(cursor + 1, last, fg, bg);
                cursor += 2;
                continue;
                
            }
            
            self.print_line(cursor, line, fg, bg);
            cursor += 1;
        }
    }
}
