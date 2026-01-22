use std::str::FromStr;
use log::info;
use ratatui::Frame;
use ratatui::layout::{Alignment, Constraint, Margin, Rect};
use ratatui::prelude::{Direction, Layout};
use ratatui::style::{Color, Modifier, Style, Styled, Stylize};
use ratatui::{widgets};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Cell, Paragraph, Row, Table, TableState};
use crate::State;

/// Key bindings.
const KEY_BINDINGS: &[(&str, &str)] = &[
    ("Search", "^f"),
    ("Up/Down", "↑/↓"),
    ("Page", "^↑/^↓"),
    ("Quit", "q"),
];

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
    // render outside block first
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

    // render inside the outside block, in this case, the content itself.
    render_viewer_content(state, frame, own_chunk.inner(Margin::new(2, 2)));
}

fn render_viewer_content(state: &mut State, frame: &mut Frame, chunk: Rect) {
    let viewport = usize::from(chunk.height).max(1);
    state.page_size = viewport;

    let selected_index = state.item_list.state.selected().unwrap_or(0);
    if selected_index < state.scroll_offset {
        state.scroll_offset = selected_index;
    }
    let bottom = state.scroll_offset.saturating_add(viewport - 1);
    if selected_index > bottom {
        state.scroll_offset = selected_index.saturating_sub(viewport - 1);
    }

    let start = state.scroll_offset;
    let end = (start + viewport).min(state.item_list.items.len());
    let items = state.item_list.items[start..end].iter();

    let mut list_state = TableState::default();
    if !state.item_list.items.is_empty() {
        list_state.select(Some(selected_index.saturating_sub(start)));
    }
    frame.render_stateful_widget(
        Table::new(
            items.map(|item| {
                Row::new(vec![Cell::from({
                    // can become a vector instead of str if needed (for styling for example)
                    Line::from(item.as_str().fg(Color::Rgb(255, 255, 255)))
                })])
            }),
            &[Constraint::Percentage(100)],
        )
        .highlight_style(Style::default()
            .fg(Color::White)
            .bg(Color::Rgb(60, 60, 60))
            .add_modifier(Modifier::BOLD))
        .highlight_symbol("> "),
        chunk,
        &mut list_state
    );
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
