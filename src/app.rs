use crate::process::ProcessState;
use std::time::Duration;

pub enum Focus {
    Tab,
    ProcessInput,
    ProcessList,
}

pub struct App<'a> {
    pub title: &'a str,
    pub tabs: TabsState<'a>,
    pub should_quit: bool,
    pub focus: Focus,
    pub process: ProcessState,
}

impl<'a> App<'a> {
    pub fn new(title: &'a str) -> App<'a> {
        App {
            title,
            tabs: TabsState::new(vec!["process", "tab1", "tab2"]),
            should_quit: false,
            focus: Focus::Tab,
            process: ProcessState::new(),
        }
    }
    pub fn on_key(&mut self, c: char) {
        match self.focus {
            Focus::ProcessInput => {
                self.process.procname.push(c);
                self.process.refresh();
            }
            Focus::ProcessList => {
                self.process.content.state.select(None);
                self.focus = Focus::ProcessInput;
            }
            _ => {
                if c == 'q' {
                    self.should_quit = true;
                }
            }
        }
    }
    pub fn on_backspace(&mut self) {
        if let Focus::ProcessInput = self.focus {
            self.process.procname.pop();
            self.process.refresh();
        }
    }
    pub fn on_up(&mut self) {
        match self.focus {
            Focus::ProcessInput => self.focus = Focus::Tab,
            Focus::ProcessList => match self.process.content.state.selected() {
                Some(x) => {
                    if x == 0 {
                        self.process
                            .content
                            .state
                            .select(Some(self.process.content.items.len() - 1));
                    } else {
                        self.process.content.state.select(Some(x - 1));
                    }
                }
                None => {}
            },
            _ => {}
        }
    }
    pub fn on_down(&mut self) {
        match self.focus {
            Focus::Tab => match self.tabs.index {
                0 => self.focus = Focus::ProcessInput,
                _ => {}
            },
            Focus::ProcessInput => {
                self.focus = Focus::ProcessList;
                self.process.content.state.select(Some(0));
            }
            Focus::ProcessList => match self.process.content.state.selected() {
                Some(x) => {
                    if x >= self.process.content.items.len() - 1 {
                        self.process.content.state.select(Some(0));
                    } else {
                        self.process.content.state.select(Some(x + 1));
                    }
                }
                None => {}
            },
        }
    }
    pub fn on_left(&mut self) {
        match self.focus {
            Focus::Tab => self.tabs.prev(),
            _ => {}
        }
    }
    pub fn on_right(&mut self) {
        match self.focus {
            Focus::Tab => self.tabs.next(),
            _ => {}
        }
    }
    pub fn on_tick(&mut self, tick_rate: &Duration) {
        self.process.tick += *tick_rate;
        if self.process.tick >= Duration::from_secs(1) {
            self.process.refresh();
            self.process.tick = Duration::from_secs(0);
        }
    }
}

pub struct TabsState<'a> {
    pub titles: Vec<&'a str>,
    pub index: usize,
}

impl<'a> TabsState<'a> {
    pub fn new(titles: Vec<&'a str>) -> TabsState {
        TabsState { titles, index: 0 }
    }
    pub fn next(&mut self) {
        self.index = (self.index + 1) % self.titles.len();
    }
    pub fn prev(&mut self) {
        if self.index > 0 {
            self.index -= 1;
        } else {
            self.index = self.titles.len() - 1;
        }
    }
}
