use std::{fs, io};

#[derive(Debug)]
pub enum AppStates {
    Running,
    TypeCmd,
}

pub struct App<'a> {
    pub title: &'a str,
    pub should_quit: bool,
    pub current_id: usize,
    pub query: String,
    pub file: String,
    pub file_contents: String,
    pub line_count: usize,
    pub state: AppStates,
    pub cmd_str: String,
}

impl<'a> App<'a> {
    pub fn new(title: &'a str, query: String, filename: String) -> Result<App<'a>, io::Error> {
        let file_contents = fs::read_to_string(filename.clone())?;
        let mut line_count = 0;
        for _ in file_contents.lines() {
            line_count += 1;
        }
        Ok(App {
            title,
            should_quit: false,
            current_id: 0,
            query: query.clone(),
            file: filename.clone(),
            file_contents: file_contents,
            line_count: line_count,
            state: AppStates::Running,
            cmd_str: "".to_string(),
        })
    }

    pub fn on_up(&mut self) {
        if self.current_id > 1 {
            self.current_id -= 1
        }
    }

    pub fn on_down(&mut self) {
        if self.current_id < self.line_count - 1 {
            self.current_id += 1
        }
    }

    pub fn on_key(&mut self, c: char) {
        match self.state {
            AppStates::Running => {
                match c {
                    'q' => {
                        self.should_quit = true;
                    }
                    'j' => {
                        self.on_down();
                    }
                    'k' => {
                        self.on_up();
                    }
                    '/' => {
                        self.state = AppStates::TypeCmd;
                    }
                    _ => {}
                }
            }
            AppStates::TypeCmd => {
                self.cmd_str.push(c);
            }
        }
    }

    pub fn on_enter(&mut self) {
        match self.state {
            AppStates::Running => {}
            AppStates::TypeCmd => {
                self.state = AppStates::Running;
            }
        }
    }
    pub fn on_tick(&mut self) {}
}
