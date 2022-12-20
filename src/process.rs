use crate::app::{App, Focus};
use std::{process::Command, time::Duration};
use tui::{
    backend::Backend,
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::Spans,
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
    Frame,
};
pub struct StatefulList {
    pub state: ListState,
    pub items: Vec<String>,
}
pub struct ProcessState {
    pub procname: String,
    pub tick: Duration,
    pub content: StatefulList,
}

impl ProcessState {
    pub fn new() -> ProcessState {
        let mut process_state = ProcessState {
            procname: String::new(),
            tick: Duration::from_secs(0),
            content: StatefulList {
                state: ListState::default(),
                items: vec!["list1".to_owned(), "list2".to_owned()],
            },
        };
        process_state.refresh();
        process_state
    }
    pub fn refresh(&mut self) {
        self.content.items.clear();
        match Command::new("ps").arg("-ef").output() {
            Ok(output) => match String::from_utf8(output.stdout) {
                Ok(str) => {
                    for (index, line) in str.lines().enumerate() {
                        if index == 0 || line.contains(&self.procname) {
                            self.content.items.push(line.to_owned());
                        }
                    }
                }
                Err(err) => {
                    self.content
                        .items
                        .push("Failed to convert output to utf8 string, msg:".to_owned());
                    self.content.items.push(err.to_string());
                }
            },
            Err(err) => {
                self.content
                    .items
                    .push("Failed to exec ps -ef, err msg:".to_owned());
                self.content.items.push(err.to_string());
            }
        }
    }
}

pub fn draw_process<B>(f: &mut Frame<B>, app: &mut App, area: Rect)
where
    B: Backend,
{
    let chunks = Layout::default()
        .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
        .split(area);

    let input_box_style = match app.focus {
        Focus::ProcessInput => Style::default().fg(Color::Red),
        _ => Style::default(),
    };

    let input_box = Paragraph::new(app.process.procname.as_ref()).block(
        Block::default()
            .title("input process name: ")
            .borders(Borders::ALL)
            .border_style(input_box_style),
    );
    f.render_widget(input_box, chunks[0]);

    let items: Vec<ListItem> = app
        .process
        .content
        .items
        .iter()
        .map(|i| {
            let lines = vec![Spans::from(i.as_ref())];
            ListItem::new(lines)
        })
        .collect();

    let list_box_style = match app.focus {
        Focus::ProcessList => Style::default().fg(Color::Red),
        _ => Style::default(),
    };

    let items_list = List::new(items)
        .block(
            Block::default()
                .title("processes, press q to exit")
                .borders(Borders::ALL)
                .border_style(list_box_style),
        )
        .highlight_style(
            Style::default()
                .bg(Color::LightGreen)
                .add_modifier(Modifier::BOLD),
        );
    f.render_stateful_widget(items_list, chunks[1], &mut app.process.content.state);
}
