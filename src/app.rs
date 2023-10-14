use std::{fs, io};

#[derive(Debug)]
pub enum AppStates {
    Running,
    FindStr,
    TypeCmd,
}

pub struct Finder {
    pub found_items: Vec<(usize, usize)>,
    pub idx: Option<usize>,
    pub word: Option<String>,
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
    pub finder: Finder,
}

impl Finder {
    pub fn new() -> Result<Finder, io::Error> {
        Ok(Finder{
            found_items: Vec::new(),
            idx: None,
            word: None,
        })
    }

    pub fn search_case_insensitive_all(&mut self, contents: & Vec<String>) -> Option<bool>{
        let mut found: bool = false;
        let mut idx = 0;

        let query = self.word.clone().unwrap().to_lowercase();

        for line in contents {
            if let Some(index) = line.to_lowercase().find(&query) {
                found = true;
                self.found_items.push((idx, index));
            }
            idx+=1;
        }

        Some(found)
    }
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
            finder: Finder::new().unwrap(),
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
                    'n' => {
                        if let Some(mut cur_id) = self.finder.idx {
                            if cur_id+1 < self.finder.found_items.len() {
                                cur_id+=1;
                                self.finder.idx = Some(cur_id);
                            }
                            self.current_id = self.finder.found_items[cur_id].0;
                        } else {
                            self.finder.idx = Some(0);
                            self.current_id = self.finder.found_items[0].0;
                        }
                    }
                    'N' => {
                        if let Some(mut cur_id) = self.finder.idx {
                            if cur_id-1 > 0 {
                                cur_id-=1;
                                self.finder.idx = Some(cur_id);
                            }
                            self.current_id = self.finder.found_items[cur_id].0;
                        } else {
                            self.finder.idx = Some(0);
                            self.current_id = self.finder.found_items[0].0;
                        }
                    }
                    '/' => {
                        self.cmd_str = "".to_string();
                        self.state = AppStates::FindStr;
                    }
                    ':' => {
                        self.state = AppStates::TypeCmd;
                    }
                    _ => {}
                }
            }
            AppStates::TypeCmd | AppStates::FindStr => {
                self.cmd_str.push(c);
            }
        }
    }

    pub fn on_enter(&mut self) {
        match self.state {
            AppStates::Running => {}
            AppStates::FindStr => {
                self.finder.word = Some(self.cmd_str.clone());
                self.finder.search_case_insensitive_all(&self.adjusted_file_contents);
                self.state = AppStates::Running;
            }
            AppStates::TypeCmd => {
                self.state = AppStates::Running;
            }
        }
    }

    pub fn on_tick(&mut self) {}
}
