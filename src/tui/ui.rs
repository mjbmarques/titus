use std::ptr::addr_of_mut;
use std::str::FromStr;
use log::{debug, info};
use ratatui::Frame;
use ratatui::layout::{Alignment, Constraint, Margin, Rect};
use ratatui::prelude::{Direction, Layout};
use ratatui::style::{Color, Modifier, Style, Styled, Stylize};
use ratatui::{widgets};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Cell, List, ListItem, ListState, Paragraph, Row, Table, TableState};
use crate::State;
use crate::tui::file_io;
use crate::tui::widgets::list::SelectableList;
use crate::tui::colors;

/// Key bindings.
const KEY_BINDINGS: &[(&str, &str)] = &[
    ("Search", "^f"),
    ("Up/Down", "↑/↓"),
    ("Page", "^↑/^↓"),
    ("Quit", "q"),
];

const LIST_BLOCK_SIZE: usize = 100;

pub fn render(state: &mut State, frame: &mut Frame) {
    info!("render state: {:?}", state);

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
                " | ".fg(colors::MEDIUM_GRAY),
                "titus".fg(colors::LIGHT_PURPLE),
                " | ".fg(colors::MEDIUM_GRAY)
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
    // render outside block first
    let block = Block::new()
        .bold()
        .fg(Color::White)
        .borders(widgets::Borders::ALL)
        .title(vec![
            " | ".fg(colors::MEDIUM_GRAY),
            "log view".fg(colors::LIGHT_GRAY),
            " | ".fg(colors::MEDIUM_GRAY)
        ])
        .title_alignment(Alignment::Center)
        .title_bottom(generate_footer_text());
    frame.render_widget(block, own_chunk);
    
    render_viewer_content(state, frame, own_chunk.inner(Margin::new(2, 2)));
}

fn render_viewer_content(state: &mut State, frame: &mut Frame, chunk: Rect) {
    log::info!("render viewer content");
    // let selected_index = state.item_list.state.selected().unwrap_or_default();
    // let page = selected_index / LIST_BLOCK_SIZE;
    // let items = state
    //     .item_list
    //     .items
    //     .iter()
    //     .skip(page * LIST_BLOCK_SIZE)
    //     .take(LIST_BLOCK_SIZE);

    // list_state.select(Some(selected_index % LIST_BLOCK_SIZE)); // should I really do this?
    // let list_items = items.map(|item| {
    //     Row::new(vec![Cell::from({
    //         // can become a vector instead of str if needed (for styling for example)
    //         Line::from(item.as_str().fg(colors::WHITE))
    //     })])
    // });
    frame.render_stateful_widget(
        List::new(state.item_list.items.iter().map(|item| ListItem::new(item.as_str())))
            // .style(colors::DIM_YELLOW)
            // .fg(colors::LIGHT_BLUE)
            // .bg(colors::STRONG_YELLOW)
            // .add_modifier(Modifier::DIM)
            .add_modifier(Modifier::BOLD)
            .highlight_style(Style::default()
                .add_modifier(Modifier::BOLD)
                .add_modifier(Modifier::REVERSED)
                .bg(colors::DIM_YELLOW))
            .highlight_symbol("> "),
        chunk,
        &mut state.item_list.state
    );
}

fn generate_footer_text() -> Line<'static> {
    let separator = Style::default()
        .fg(colors::WEAK_BLUE);
    let footer = Style::default()
        .bg(colors::BLACK_BLUE)
        .fg(colors::LIGHT_BLUE);

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