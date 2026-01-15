use std::str::FromStr;
use log::info;
use ratatui::Frame;
use ratatui::layout::Constraint;
use ratatui::prelude::{Direction, Layout, Line};
use ratatui::style::{Color, Style, Styled, Stylize};
use ratatui::{widgets};
use ratatui::text::Span;
use ratatui::widgets::Paragraph;
use crate::State;
use crate::tui::file_io;

/// Key bindings.
const KEY_BINDINGS: &[(&str, &str)] = &[
    ("Search", "^f"),
    ("Up/Down", "↑/↓"),
    ("Quit", "q"),
];

pub fn render(state: &State, frame: &mut Frame) {
    info!("render state: {:?}", state);
    let outer_layout = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(vec![
            Constraint::Percentage(100)
        ])
        .split(frame.area());

    let block = widgets::Block::new()
        .bold()
        .fg(Color::Blue)
        .borders(widgets::Borders::ALL)
        .title("titus")
        .title_alignment(ratatui::layout::Alignment::Center)
        .title_bottom(generate_footer_text());

    let paragraph = Paragraph::new(get_test_lines())
        .block(block)
        .scroll((state.scroll_bar_position.get_position() as u16, 0));



    frame.render_widget(paragraph, outer_layout[0]);
}

fn get_test_lines() -> Vec<Line<'static>> { // TODO: change to a real approach; this is temporary..
    let mut lines_vec: Vec<Line> = Vec::new();
    let file_location =  "./log/titus-2025-12-30_15-10-47.log";
    if let Ok(lines ) = file_io::read_lines(file_location) {
        for line in lines.map_while(Result::ok) {
            lines_vec.push(Line::from(line));
        }
    };
    lines_vec
}

fn generate_footer_text() -> Line<'static> {
    let separator = Style::default()
        .fg(Color::from_str("#6272A4")
            .expect("failed to get color from string"));
    let footer = Style::default()
        .bg(Color::from_str("#282A36").expect("failed to get color from string"))
        .fg(Color::from_str("#8BE9FD").expect("failed to get color from string"));

    let line = KEY_BINDINGS.iter()
        .enumerate()
        .flat_map(|(i, (keys, desc))| {
            vec![
                "<".set_style(separator),
                keys.set_style(footer),
                ": ".set_style(separator),
                Span::from(*desc).set_style(footer),
                ">".set_style(separator),
                if i != KEY_BINDINGS.len() - 1 { " " } else { "" }.into(),
            ]
        })
        .collect::<Vec<Span>>();

    Line::from(line).centered()
}