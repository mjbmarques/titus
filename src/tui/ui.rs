use log::{debug, error, info};
use ratatui::Frame;
use ratatui::layout::{Alignment, Constraint, Margin, Rect};
use ratatui::prelude::{Direction, Layout};
use ratatui::style::{Color, Modifier, Style, Styled, Stylize};
use ratatui::{widgets};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, List, ListItem, Paragraph};
use crate::State;
use crate::tui::colors;
use crate::tui::state::Dimensions;

/// Key bindings.
const KEY_BINDINGS: &[(&str, &str)] = &[
    ("Search", "^f"),
    ("Nav", "↑/↓"),
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

    if state.curr_open_file.is_some() {
        frame.render_widget(
            Paragraph::new(state.curr_open_file.as_ref().unwrap().file_name.as_str()),
            layout[0]
        );
    }


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
    info!("render viewer content");
    update_mode_dimensions(state, chunk);
    error!("[TEST] - chunk height: {}, width: {}", chunk.height, chunk.width);
    frame.render_stateful_widget(
        List::new(state.current_list.items.iter().map(|item| ListItem::new(item.as_str())))
            .add_modifier(Modifier::BOLD)
            .highlight_style(Style::default()
                .add_modifier(Modifier::BOLD)
                .add_modifier(Modifier::REVERSED)
                .bg(colors::DIM_YELLOW))
            .highlight_symbol("> "),
        chunk,
        &mut state.current_list.state
    );
}

fn update_mode_dimensions(state: &mut State, chunk: Rect) {
    state.current_mode.dimensions = Dimensions {
        width: chunk.width,
        height: chunk.height,
    };
    debug!("Updated mode {:?} with dimensions: {:?}",
        state.current_mode.mode, state.current_mode.dimensions);
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