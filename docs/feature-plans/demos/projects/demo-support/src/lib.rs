use ratatui::backend::TestBackend;
use ratatui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Clear, List, ListItem, Paragraph, Wrap};
use ratatui::{Frame, Terminal};
use std::fs;
use std::path::Path;

#[derive(Clone, Copy)]
pub struct Theme {
    pub bg: Color,
    pub fg: Color,
    pub accent: Color,
    pub muted: Color,
    pub warning: Color,
}

pub struct Panel<'a> {
    pub title: &'a str,
    pub lines: Vec<&'a str>,
}

pub struct DemoConfig<'a> {
    pub header_text: &'a str,
    pub file_line: Option<&'a str>,
    pub footer_text: Option<&'a str>,
    pub log_title: &'a str,
    pub log_lines: Vec<&'a str>,
    pub left_panel: Option<Panel<'a>>,
    pub top_panel: Option<Panel<'a>>,
    pub overlay: Option<Panel<'a>>,
    pub overlay_footer: Option<&'a str>,
    pub theme: Theme,
}

pub struct DemoRenderer {
    terminal: Terminal<TestBackend>,
}

impl DemoRenderer {
    pub fn new(width: u16, height: u16) -> Self {
        let backend = TestBackend::new(width, height);
        let terminal = Terminal::new(backend).expect("terminal");
        Self { terminal }
    }

    pub fn render(&mut self, config: &DemoConfig<'_>) {
        self.terminal
            .draw(|frame| render_demo(frame, config))
            .expect("draw");
    }

    pub fn write_svg(&self, output: impl AsRef<Path>) {
        let buffer = self.terminal.backend().buffer();
        let svg = buffer_to_svg(buffer);
        if let Some(parent) = output.as_ref().parent() {
            fs::create_dir_all(parent).expect("mkdir");
        }
        fs::write(output, svg).expect("write svg");
    }
}

pub fn render_demo(frame: &mut Frame, config: &DemoConfig<'_>) {
    let area = frame.area();
    frame.render_widget(
        Block::default().style(Style::default().bg(config.theme.bg).fg(config.theme.fg)),
        area,
    );

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0), Constraint::Length(3)])
        .split(area);

    render_header(frame, layout[0], config);
    render_body(frame, layout[1], config);
    render_footer(frame, layout[2], config);

    if let Some(overlay) = &config.overlay {
        render_overlay(frame, area, overlay, config.overlay_footer, config.theme);
    }
}

fn render_header(frame: &mut Frame, area: Rect, config: &DemoConfig<'_>) {
    let header = Paragraph::new(config.header_text)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(Span::styled("titus", Style::default().fg(config.theme.accent)))
                .title_alignment(Alignment::Center)
                .style(Style::default().fg(config.theme.fg).bg(config.theme.bg)),
        )
        .alignment(Alignment::Center);
    frame.render_widget(header, area);
}

fn render_body(frame: &mut Frame, area: Rect, config: &DemoConfig<'_>) {
    let mut constraints = Vec::new();
    if config.file_line.is_some() {
        constraints.push(Constraint::Length(1));
    }
    if config.top_panel.is_some() {
        constraints.push(Constraint::Length(3));
    }
    constraints.push(Constraint::Min(0));

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(constraints)
        .split(area);

    let mut index = 0;
    if let Some(file_line) = config.file_line {
        let file_para = Paragraph::new(file_line)
            .style(Style::default().fg(config.theme.muted))
            .alignment(Alignment::Left);
        frame.render_widget(file_para, chunks[index]);
        index += 1;
    }

    if let Some(panel) = &config.top_panel {
        render_panel(frame, chunks[index], panel, config.theme);
        index += 1;
    }

    let log_area = chunks[index];
    if let Some(left_panel) = &config.left_panel {
        let split = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(32), Constraint::Percentage(68)])
            .split(log_area);
        render_panel(frame, split[0], left_panel, config.theme);
        render_log_list(frame, split[1], config);
    } else {
        render_log_list(frame, log_area, config);
    }
}

fn render_log_list(frame: &mut Frame, area: Rect, config: &DemoConfig<'_>) {
    let items: Vec<ListItem> = config
        .log_lines
        .iter()
        .map(|line| ListItem::new(*line))
        .collect();
    let list = List::new(items).block(
        Block::default()
            .borders(Borders::ALL)
            .title(Span::styled(
                config.log_title,
                Style::default().fg(config.theme.muted),
            ))
            .title_alignment(Alignment::Center),
    );
    frame.render_widget(list, area);
}

fn render_footer(frame: &mut Frame, area: Rect, config: &DemoConfig<'_>) {
    let footer = Paragraph::new(config.footer_text.unwrap_or(""))
        .block(Block::default().borders(Borders::ALL))
        .style(Style::default().fg(config.theme.muted))
        .alignment(Alignment::Center);
    frame.render_widget(footer, area);
}

fn render_panel(frame: &mut Frame, area: Rect, panel: &Panel<'_>, theme: Theme) {
    let lines: Vec<Line> = panel
        .lines
        .iter()
        .map(|line| Line::from(Span::styled(*line, Style::default().fg(theme.fg))))
        .collect();
    let paragraph = Paragraph::new(lines)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(Span::styled(panel.title, Style::default().fg(theme.accent))),
        )
        .wrap(Wrap { trim: true });
    frame.render_widget(paragraph, area);
}

fn render_overlay(
    frame: &mut Frame,
    area: Rect,
    panel: &Panel<'_>,
    footer: Option<&str>,
    theme: Theme,
) {
    let height = panel.lines.len() as u16 + if footer.is_some() { 4 } else { 3 };
    let popup_area = centered_rect(60, height, area);
    frame.render_widget(Clear, popup_area);

    let mut lines: Vec<Line> = panel
        .lines
        .iter()
        .map(|line| Line::from(Span::styled(*line, Style::default().fg(theme.fg))))
        .collect();
    if let Some(footer_text) = footer {
        lines.push(Line::from(Span::styled(
            footer_text,
            Style::default().fg(theme.warning),
        )));
    }

    let paragraph = Paragraph::new(lines).block(
        Block::default()
            .borders(Borders::ALL)
            .title(Span::styled(panel.title, Style::default().fg(theme.accent))),
    );
    frame.render_widget(paragraph, popup_area);
}

fn centered_rect(percent_x: u16, height: u16, rect: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(50),
            Constraint::Length(height),
            Constraint::Percentage(50),
        ])
        .split(rect);
    let horizontal = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1]);
    horizontal[1]
}

fn buffer_to_svg(buffer: &ratatui::buffer::Buffer) -> String {
    let width = buffer.area.width as u32;
    let height = buffer.area.height as u32;
    let _font_size = 16.0;
    let cell_width = 9.0;
    let cell_height = 18.0;
    let svg_width = (width as f32 * cell_width + 20.0).ceil();
    let svg_height = (height as f32 * cell_height + 20.0).ceil();

    let mut svg = String::new();
    svg.push_str(&format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{svg_width}\" height=\"{svg_height}\" viewBox=\"0 0 {svg_width} {svg_height}\">\n"
    ));
    svg.push_str("<rect width=\"100%\" height=\"100%\" fill=\"#000000\"/>\n");
    svg.push_str("<g font-family=\"DejaVu Sans Mono\" font-size=\"16\">\n");

    for y in 0..height {
        for x in 0..width {
            let cell = buffer.cell((x as u16, y as u16)).unwrap();
            let symbol = cell.symbol();
            if symbol.is_empty() {
                continue;
            }
            let fg = color_to_hex(cell.fg, "#F8F8F2");
            let bg = color_to_hex(cell.bg, "#000000");
            let pos_x = 10.0 + x as f32 * cell_width;
            let pos_y = 10.0 + (y + 1) as f32 * cell_height - 4.0;
            svg.push_str(&format!(
                "<rect x=\"{pos_x}\" y=\"{}\" width=\"{cell_width}\" height=\"{cell_height}\" fill=\"{bg}\"/>\n",
                pos_y - cell_height + 4.0
            ));
            svg.push_str(&format!(
                "<text x=\"{pos_x}\" y=\"{pos_y}\" fill=\"{fg}\">{}</text>\n",
                escape_svg(symbol)
            ));
        }
    }
    svg.push_str("</g>\n</svg>\n");
    svg
}

fn color_to_hex(color: Color, fallback: &str) -> String {
    match color {
        Color::Rgb(r, g, b) => format!("#{r:02X}{g:02X}{b:02X}"),
        Color::Reset => fallback.to_string(),
        Color::Indexed(_) => fallback.to_string(),
        _ => fallback.to_string(),
    }
}

fn escape_svg(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

pub fn sample_log_lines() -> Vec<&'static str> {
    vec![
        "• 12052: INFO  Starting service",
        "• 12053: INFO  Listening on :8080",
        "• 12054: WARN  Disk usage high",
        "• 12055: INFO  Health check OK",
        "• 12056: INFO  Ready to accept traffic",
    ]
}

pub fn theme_dracula() -> Theme {
    Theme {
        bg: Color::Rgb(40, 42, 54),
        fg: Color::Rgb(248, 248, 242),
        accent: Color::Rgb(189, 147, 249),
        muted: Color::Rgb(98, 114, 164),
        warning: Color::Rgb(255, 184, 108),
    }
}

pub fn theme_solarized() -> Theme {
    Theme {
        bg: Color::Rgb(0, 43, 54),
        fg: Color::Rgb(131, 148, 150),
        accent: Color::Rgb(38, 139, 210),
        muted: Color::Rgb(88, 110, 117),
        warning: Color::Rgb(181, 137, 0),
    }
}

pub fn theme_gruvbox() -> Theme {
    Theme {
        bg: Color::Rgb(40, 40, 40),
        fg: Color::Rgb(235, 219, 178),
        accent: Color::Rgb(254, 128, 25),
        muted: Color::Rgb(146, 131, 116),
        warning: Color::Rgb(250, 189, 47),
    }
}

pub fn theme_catppuccin() -> Theme {
    Theme {
        bg: Color::Rgb(30, 30, 46),
        fg: Color::Rgb(205, 214, 244),
        accent: Color::Rgb(203, 166, 247),
        muted: Color::Rgb(108, 112, 134),
        warning: Color::Rgb(249, 226, 175),
    }
}
