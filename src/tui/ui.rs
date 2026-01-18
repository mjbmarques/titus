use std::ptr::addr_of_mut;
use std::str::FromStr;
use log::info;
use ratatui::Frame;
use ratatui::layout::{Alignment, Constraint, Margin, Rect};
use ratatui::prelude::{Direction, Layout};
use ratatui::style::{Color, Style, Styled, Stylize};
use ratatui::{widgets};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Cell, Paragraph, Row, Table, TableState};
use crate::State;
use crate::tui::file_io;
use crate::tui::widgets::list::SelectableList;

/// Key bindings.
const KEY_BINDINGS: &[(&str, &str)] = &[
    ("Search", "^f"),
    ("Up/Down", "↑/↓"),
    ("Quit", "q"),
];

const LIST_BLOCK_SIZE: usize = 100;

pub fn render(state: &mut State, frame: &mut Frame) {
    info!("render state: {:?}", state);
    // let outer_layout = Layout::default()
    //     .direction(Direction::Vertical)
    //     .margin(1)
    //     .constraints(vec![
    //         Constraint::Percentage(100)
    //     ])
    //     .split(frame.area());

    let layout = Layout::new(
        Direction::Vertical,
        [Constraint::Length(3), Constraint::Min(0)],
    )
        .direction(Direction::Vertical)
        .margin(1)
        .split(frame.area());

    render_main_header(state, frame, layout[0]);
    render_main_log_view(state, frame, layout[1]);
}

fn render_main_header(state: &State, frame: &mut Frame, own_chunk: Rect) {
    frame.render_widget(
        Block::bordered()
            .title(vec![
                " | ".fg(Color::Rgb(100, 100, 100)),
                "titus".fg(Color::Rgb(90, 50, 168)),
                " | ".fg(Color::Rgb(100, 100, 100))
            ])
            .title_alignment(Alignment::Center),
        own_chunk
    );

    let layout = Layout::new(
        Direction::Horizontal,
        [Constraint::Percentage(100)],
    )
        .direction(Direction::Horizontal)
        .margin(1)
        .split(own_chunk);

    let file_name = state.curr_open_file.as_deref().unwrap_or("empty");
    frame.render_widget(
        Paragraph::new(file_name),
        layout[0]
    );


}
fn render_main_log_view(state: &mut State, frame: &mut Frame, own_chunk: Rect) {
    let block = Block::new()
        .bold()
        .fg(Color::White)
        .borders(widgets::Borders::ALL)
        .title(vec![
            " | ".fg(Color::Rgb(100, 100, 100)),
            "log view".fg(Color::Rgb(150, 150, 150)),
            " | ".fg(Color::Rgb(100, 100, 100))
        ])
        .title_alignment(Alignment::Center)
        .title_bottom(generate_footer_text());
    frame.render_widget(block, own_chunk);

    // let paragraph = Paragraph::new(get_test_lines())
    //     .block(block)
    //     .scroll((state.scroll_bar_position.get_position() as u16, 0));
    //
    // frame.render_widget(paragraph, own_chunk);
    state.item_list = SelectableList::with_items(get_test_strings(state));
    render_viewer_content(state, frame, own_chunk.inner(Margin::new(2, 2)));
}

fn render_viewer_content(state: &State, frame: &mut Frame, chunk: Rect) {
    let selected_index = state.item_list.state.selected().unwrap_or_default();
    let page = selected_index / LIST_BLOCK_SIZE;
    let items = state
        .item_list
        .items
        .iter()
        .skip(page * LIST_BLOCK_SIZE)
        .take(LIST_BLOCK_SIZE);

    let mut list_state = TableState::default();
    list_state.select(Some(selected_index % LIST_BLOCK_SIZE));
    frame.render_stateful_widget(
        Table::new(
            items.map(|item| {
                Row::new(vec![Cell::from({
                    // can become a vector instead of str if needed (for styling for example)
                    Line::from(item.as_str().fg(Color::Rgb(255, 255, 255)))
                })])
            }),
            &[Constraint::Percentage(100)],
        ),
        chunk,
        &mut list_state
    );
}

// For testing purposes only; to be replaced with real file opening logic, and in the right place
// which is not here at all.
fn get_test_strings(state: &mut State) -> Vec<String> {
    let mut lines_vec: Vec<String> = Vec::new();
    let file_location =  "./log/titus-2025-12-30_15-10-47.log";
    if let Ok(lines ) = file_io::read_lines(file_location) {
        for line in lines.map_while(Result::ok) {
            lines_vec.push(line);
        }
    };
    if lines_vec.len() > 0 {
        state.curr_open_file = Some(file_location.to_string());
    }
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