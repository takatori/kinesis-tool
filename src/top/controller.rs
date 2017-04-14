use std::process;

use utils::screen::Screen;
use utils::screen::ScreenStatus;
use aws::awsController;

pub struct topController {
    screen: Screen,
}

impl topController {

    pub fn new() -> topController {
        topController {
            screen: Screen::new(),
        }
    }

    pub fn run(&mut self) {
        let commands = vec!["a".to_string(), "q".to_string()];
        self.screen.update_screen(r###"
   -----------------------------------------

   ██████╗  █████╗ ██╗    ██╗███████╗████████╗
   ██╔══██╗██╔══██╗██║    ██║██╔════╝╚══██╔══╝
   ██████╔╝███████║██║ █╗ ██║███████╗   ██║   
   ██╔══██╗██╔══██║██║███╗██║╚════██║   ██║   
   ██║  ██║██║  ██║╚███╔███╔╝███████║   ██║   
   ╚═╝  ╚═╝╚═╝  ╚═╝ ╚══╝╚══╝ ╚══════╝   ╚═╝   

   ------------------------------------------                                           

   rawst is the interactive tool for AWS CLI written in Rust.
   "###, &commands);

        match self.screen.select_line() {
            ScreenStatus::Error | ScreenStatus::Quit => println!("error"),
            ScreenStatus::Selected(ref c) if c == "a" => self.aws(),
            _ => process::exit(0),
        }
    }

    fn aws(&self) {
        let mut aws = awsController::new();
        aws.run();
    }
}
