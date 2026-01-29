use ratatui::backend::TestBackend;
use ratatui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Clear, List, ListItem, Paragraph, Wrap};
use ratatui::Terminal;
use std::env;
use std::fs;
use std::path::Path;

#[derive(Clone, Copy)]
struct Theme {
    bg: Color,
    fg: Color,
    accent: Color,
    muted: Color,
    warning: Color,
}

struct Panel<'a> {
    title: &'a str,
    lines: Vec<&'a str>,
}

struct DemoConfig<'a> {
    header_text: &'a str,
    file_line: Option<&'a str>,
    footer_text: Option<&'a str>,
    log_title: &'a str,
    log_lines: Vec<&'a str>,
    left_panel: Option<Panel<'a>>,
    top_panel: Option<Panel<'a>>,
    overlay: Option<Panel<'a>>,
    overlay_footer: Option<&'a str>,
    theme: Theme,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: demo_renderer <demo-name> <output-json>");
        std::process::exit(1);
    }
    let demo = args[1].as_str();
    let output = Path::new(&args[2]);
    let config = demo_config(demo).unwrap_or_else(|| {
        eprintln!("Unknown demo: {demo}");
        std::process::exit(1);
    });

    let backend = TestBackend::new(96, 28);
    let mut terminal = Terminal::new(backend).expect("terminal");
    terminal
        .draw(|frame| render_demo(frame, &config))
        .expect("draw");

    let buffer = terminal.backend().buffer();
    let output_json = buffer_to_json(buffer, config.theme);
    if let Some(parent) = output.parent() {
        fs::create_dir_all(parent).expect("mkdir");
    }
    fs::write(output, output_json).expect("write output");
}

fn demo_config(name: &str) -> Option<DemoConfig<'static>> {
    let default_theme = theme_dracula();
    match name {
        "file-import-modal" => Some(DemoConfig {
            header_text: "titus",
            file_line: Some("File: /var/log/system.log"),
            footer_text: Some("<Search: ^f> <Nav: ↑/↓> <Page: ^↑/^↓> <Quit: q> <Import: ^o>"),
            log_title: "log view",
            log_lines: sample_log_lines(),
            left_panel: None,
            top_panel: None,
            overlay: Some(Panel {
                title: "Open Log File",
                lines: vec!["Path: /var/log/nginx/access.log", "[Browse] [Open] [Cancel]"],
            }),
            overlay_footer: None,
            theme: default_theme,
        }),
        "file-import-inline" => Some(DemoConfig {
            header_text: "titus",
            file_line: Some("File: /var/log/system.log"),
            footer_text: Some("<Search: ^f> <Nav: ↑/↓> <Page: ^↑/^↓> <Quit: q> <Import: ^o>"),
            log_title: "log view",
            log_lines: sample_log_lines(),
            left_panel: None,
            top_panel: Some(Panel {
                title: "Open File",
                lines: vec!["/var/log/nginx/access.log", "[Browse]"],
            }),
            overlay: None,
            overlay_footer: None,
            theme: default_theme,
        }),
        "file-import-sidebar" => Some(DemoConfig {
            header_text: "titus",
            file_line: Some("File: /var/log/system.log"),
            footer_text: Some("<Search: ^f> <Nav: ↑/↓> <Page: ^↑/^↓> <Quit: q> <Import: ^o>"),
            log_title: "log view",
            log_lines: sample_log_lines(),
            left_panel: Some(Panel {
                title: "Files",
                lines: vec![
                    "/var/log/",
                    "└ system.log",
                    "└ nginx/",
                    "  └ access.log",
                    "  └ error.log",
                    "└ auth.log",
                ],
            }),
            top_panel: None,
            overlay: None,
            overlay_footer: None,
            theme: default_theme,
        }),
        "go-to-line-modal" => Some(DemoConfig {
            header_text: "titus",
            file_line: Some("File: /var/log/system.log"),
            footer_text: Some("<Search: ^f> <Nav: ↑/↓> <Page: ^↑/^↓> <Quit: q> <Go: g>"),
            log_title: "log view",
            log_lines: sample_log_lines(),
            left_panel: None,
            top_panel: None,
            overlay: Some(Panel {
                title: "Go to Line",
                lines: vec!["Line: 12054", "[Go] [Cancel]"],
            }),
            overlay_footer: None,
            theme: default_theme,
        }),
        "go-to-line-inline" => Some(DemoConfig {
            header_text: "titus",
            file_line: Some("File: /var/log/system.log"),
            footer_text: Some("<Search: ^f> <Nav: ↑/↓> <Page: ^↑/^↓> <Quit: q> <Go: g>"),
            log_title: "log view",
            log_lines: sample_log_lines(),
            left_panel: None,
            top_panel: Some(Panel {
                title: "Jump to line",
                lines: vec!["[ 12054 ]", "Range: 1..46832"],
            }),
            overlay: None,
            overlay_footer: None,
            theme: default_theme,
        }),
        "go-to-line-status" => Some(DemoConfig {
            header_text: "titus",
            file_line: Some("File: /var/log/system.log"),
            footer_text: Some("Go to line: 12054 (Enter to jump, Esc to cancel)"),
            log_title: "log view",
            log_lines: sample_log_lines(),
            left_panel: None,
            top_panel: None,
            overlay: None,
            overlay_footer: None,
            theme: default_theme,
        }),
        "log-search-footer" => Some(DemoConfig {
            header_text: "titus",
            file_line: Some("File: /var/log/system.log"),
            footer_text: Some("Find: error (Case: off | Regex: off) [Enter: next] [Shift+Enter: prev]"),
            log_title: "log view",
            log_lines: sample_log_lines(),
            left_panel: None,
            top_panel: None,
            overlay: None,
            overlay_footer: None,
            theme: default_theme,
        }),
        "log-search-modal" => Some(DemoConfig {
            header_text: "titus",
            file_line: Some("File: /var/log/system.log"),
            footer_text: Some("<Search: ^f> <Nav: ↑/↓> <Page: ^↑/^↓> <Quit: q>"),
            log_title: "log view",
            log_lines: sample_log_lines(),
            left_panel: None,
            top_panel: None,
            overlay: Some(Panel {
                title: "Find in Logs",
                lines: vec![
                    "Query: error",
                    "[Regex] [Case] [Whole word]",
                    "[Prev] [Next] [Close]",
                ],
            }),
            overlay_footer: None,
            theme: default_theme,
        }),
        "log-search-sidebar" => Some(DemoConfig {
            header_text: "titus",
            file_line: Some("File: /var/log/system.log"),
            footer_text: Some("Find: error (Matches: 14) [Prev] [Next]"),
            log_title: "log view",
            log_lines: sample_log_lines(),
            left_panel: Some(Panel {
                title: "Matches",
                lines: vec![
                    "12054 WARN Disk usage high",
                    "12088 ERROR Database error",
                    "12102 ERROR Connection reset",
                    "12120 WARN Disk usage high",
                ],
            }),
            top_panel: None,
            overlay: None,
            overlay_footer: None,
            theme: default_theme,
        }),
        "partial-loading-windowed" => Some(DemoConfig {
            header_text: "titus",
            file_line: Some("File: /var/log/system.log (Loaded: 2000..2400 of 46832)"),
            footer_text: Some("Window: 2000..2400 [PgUp/PgDn loads] Cache: 3 windows"),
            log_title: "log view",
            log_lines: sample_log_lines(),
            left_panel: None,
            top_panel: None,
            overlay: None,
            overlay_footer: None,
            theme: default_theme,
        }),
        "partial-loading-infinite" => Some(DemoConfig {
            header_text: "titus",
            file_line: Some("File: /var/log/system.log (Loaded: 1..400)"),
            footer_text: Some("Loading more lines... Buffer: 400 (Chunk: 200)"),
            log_title: "log view",
            log_lines: sample_log_lines(),
            left_panel: None,
            top_panel: None,
            overlay: None,
            overlay_footer: None,
            theme: default_theme,
        }),
        "partial-loading-segment" => Some(DemoConfig {
            header_text: "titus",
            file_line: Some("File: /var/log/system.log (Segment 3/12: lines 10001..15000)"),
            footer_text: Some("Segment: [Prev] 3/12 [Next] Jump: g Search: ^f"),
            log_title: "log view",
            log_lines: sample_log_lines(),
            left_panel: None,
            top_panel: None,
            overlay: None,
            overlay_footer: None,
            theme: default_theme,
        }),
        "color-scheme-dracula" => Some(DemoConfig {
            header_text: "titus",
            file_line: Some("File: /var/log/system.log"),
            footer_text: Some("Palette: #282A36 #F8F8F2 #6272A4 #BD93F9"),
            log_title: "log view",
            log_lines: sample_log_lines(),
            left_panel: None,
            top_panel: None,
            overlay: None,
            overlay_footer: None,
            theme: theme_dracula(),
        }),
        "color-scheme-solarized" => Some(DemoConfig {
            header_text: "titus",
            file_line: Some("File: /var/log/system.log"),
            footer_text: Some("Palette: #002B36 #839496 #586E75 #268BD2"),
            log_title: "log view",
            log_lines: sample_log_lines(),
            left_panel: None,
            top_panel: None,
            overlay: None,
            overlay_footer: None,
            theme: theme_solarized(),
        }),
        "color-scheme-gruvbox" => Some(DemoConfig {
            header_text: "titus",
            file_line: Some("File: /var/log/system.log"),
            footer_text: Some("Palette: #282828 #EBDBB2 #928374 #FE8019"),
            log_title: "log view",
            log_lines: sample_log_lines(),
            left_panel: None,
            top_panel: None,
            overlay: None,
            overlay_footer: None,
            theme: theme_gruvbox(),
        }),
        "theme-support-selector" => Some(DemoConfig {
            header_text: "titus",
            file_line: Some("File: /var/log/system.log"),
            footer_text: Some("<Theme: ^t> <Search: ^f> <Nav: ↑/↓> <Quit: q>"),
            log_title: "log view",
            log_lines: sample_log_lines(),
            left_panel: None,
            top_panel: None,
            overlay: Some(Panel {
                title: "Theme",
                lines: vec![
                    "• Dracula (default)",
                    "• Catppuccin Mocha",
                    "• Solarized Dark",
                    "• Gruvbox Dark",
                    "[Apply] [Cancel]",
                ],
            }),
            overlay_footer: None,
            theme: theme_dracula(),
        }),
        "theme-support-config" => Some(DemoConfig {
            header_text: "titus [catppuccin-mocha]",
            file_line: Some("File: /var/log/system.log"),
            footer_text: Some("<Theme: ^t> <Search: ^f> <Nav: ↑/↓> <Quit: q>"),
            log_title: "log view",
            log_lines: sample_log_lines(),
            left_panel: Some(Panel {
                title: "config/themes.yaml",
                lines: vec![
                    "current: catppuccin-mocha",
                    "",
                    "palettes:",
                    "  catppuccin-mocha:",
                    "    background: #1E1E2E",
                    "    foreground: #CDD6F4",
                    "    accent: #CBA6F7",
                    "    warning: #F9E2AF",
                ],
            }),
            top_panel: None,
            overlay: None,
            overlay_footer: None,
            theme: theme_catppuccin(),
        }),
        "theme-support-preview" => Some(DemoConfig {
            header_text: "titus",
            file_line: Some("File: /var/log/system.log"),
            footer_text: Some("Press Enter to apply theme"),
            log_title: "log view",
            log_lines: sample_log_lines(),
            left_panel: Some(Panel {
                title: "Theme Preview",
                lines: vec![
                    "Dracula",
                    "Catppuccin Mocha",
                    "Solarized Dark",
                    "Gruvbox Dark",
                ],
            }),
            top_panel: None,
            overlay: None,
            overlay_footer: None,
            theme: theme_dracula(),
        }),
        "multiple-files-tabs" => Some(DemoConfig {
            header_text: "titus [system.log] [access.log] [error.log]",
            file_line: Some("File: /var/log/system.log"),
            footer_text: Some("<Search: ^f> <Nav: ↑/↓> <Page: ^↑/^↓> <Quit: q> <Files: alt-tab>"),
            log_title: "log view",
            log_lines: sample_log_lines(),
            left_panel: None,
            top_panel: None,
            overlay: None,
            overlay_footer: None,
            theme: default_theme,
        }),
        "multiple-files-sidebar" => Some(DemoConfig {
            header_text: "titus",
            file_line: Some("File: /var/log/system.log"),
            footer_text: Some("<Search: ^f> <Nav: ↑/↓> <Page: ^↑/^↓> <Quit: q> <Files: alt-tab>"),
            log_title: "log view",
            log_lines: sample_log_lines(),
            left_panel: Some(Panel {
                title: "Files",
                lines: vec!["• system.log", "• access.log", "• error.log", "• audit.log"],
            }),
            top_panel: None,
            overlay: None,
            overlay_footer: None,
            theme: default_theme,
        }),
        "multiple-files-dropdown" => Some(DemoConfig {
            header_text: "titus",
            file_line: Some("File: /var/log/system.log"),
            footer_text: Some("Files: system.log ▾"),
            log_title: "log view",
            log_lines: sample_log_lines(),
            left_panel: None,
            top_panel: Some(Panel {
                title: "Files",
                lines: vec!["system.log", "access.log", "error.log", "audit.log"],
            }),
            overlay: None,
            overlay_footer: None,
            theme: default_theme,
        }),
        _ => None,
    }
}

fn render_demo(frame: &mut ratatui::Frame, config: &DemoConfig<'_>) {
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

fn render_header(frame: &mut ratatui::Frame, area: Rect, config: &DemoConfig<'_>) {
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

fn render_body(frame: &mut ratatui::Frame, area: Rect, config: &DemoConfig<'_>) {
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

fn render_log_list(frame: &mut ratatui::Frame, area: Rect, config: &DemoConfig<'_>) {
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

fn render_footer(frame: &mut ratatui::Frame, area: Rect, config: &DemoConfig<'_>) {
    let footer = Paragraph::new(config.footer_text.unwrap_or(""))
        .block(Block::default().borders(Borders::ALL))
        .style(Style::default().fg(config.theme.muted))
        .alignment(Alignment::Center);
    frame.render_widget(footer, area);
}

fn render_panel(frame: &mut ratatui::Frame, area: Rect, panel: &Panel<'_>, theme: Theme) {
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
    frame: &mut ratatui::Frame,
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

fn buffer_to_json(buffer: &ratatui::buffer::Buffer, theme: Theme) -> String {
    let width = buffer.area.width;
    let height = buffer.area.height;
    let mut json = String::new();
    json.push_str(&format!(
        "{{\"width\":{width},\"height\":{height},\"cells\":["
    ));
    for y in 0..height {
        for x in 0..width {
            let cell = buffer.get(x, y);
            let fg = color_to_rgb(cell.fg, theme.fg);
            let bg = color_to_rgb(cell.bg, theme.bg);
            let symbol = cell.symbol().to_string();
            json.push_str(&format!(
                "{{\"x\":{x},\"y\":{y},\"symbol\":{symbol:?},\"fg\":\"#{:02X}{:02X}{:02X}\",\"bg\":\"#{:02X}{:02X}{:02X}\"}},",
                fg.0,
                fg.1,
                fg.2,
                bg.0,
                bg.1,
                bg.2
            ));
        }
    }
    if json.ends_with(',') {
        json.pop();
    }
    json.push_str("]}");
    json
}

fn color_to_rgb(color: Color, fallback: Color) -> (u8, u8, u8) {
    match color {
        Color::Rgb(r, g, b) => (r, g, b),
        Color::Black => (0, 0, 0),
        Color::Red => (205, 49, 49),
        Color::Green => (13, 188, 121),
        Color::Yellow => (229, 229, 16),
        Color::Blue => (36, 114, 200),
        Color::Magenta => (188, 63, 188),
        Color::Cyan => (17, 168, 205),
        Color::Gray => (128, 128, 128),
        Color::DarkGray => (80, 80, 80),
        Color::LightRed => (241, 76, 76),
        Color::LightGreen => (35, 209, 139),
        Color::LightYellow => (245, 245, 67),
        Color::LightBlue => (59, 142, 234),
        Color::LightMagenta => (214, 112, 214),
        Color::LightCyan => (41, 184, 219),
        Color::White => (255, 255, 255),
        Color::Reset => color_to_rgb(fallback, fallback),
        Color::Indexed(_) => color_to_rgb(fallback, fallback),
    }
}

fn sample_log_lines() -> Vec<&'static str> {
    vec![
        "• 12052: INFO  Starting service",
        "• 12053: INFO  Listening on :8080",
        "• 12054: WARN  Disk usage high",
        "• 12055: INFO  Health check OK",
        "• 12056: INFO  Ready to accept traffic",
    ]
}

fn theme_dracula() -> Theme {
    Theme {
        bg: Color::Rgb(40, 42, 54),
        fg: Color::Rgb(248, 248, 242),
        accent: Color::Rgb(189, 147, 249),
        muted: Color::Rgb(98, 114, 164),
        warning: Color::Rgb(255, 184, 108),
    }
}

fn theme_solarized() -> Theme {
    Theme {
        bg: Color::Rgb(0, 43, 54),
        fg: Color::Rgb(131, 148, 150),
        accent: Color::Rgb(38, 139, 210),
        muted: Color::Rgb(88, 110, 117),
        warning: Color::Rgb(181, 137, 0),
    }
}

fn theme_gruvbox() -> Theme {
    Theme {
        bg: Color::Rgb(40, 40, 40),
        fg: Color::Rgb(235, 219, 178),
        accent: Color::Rgb(254, 128, 25),
        muted: Color::Rgb(146, 131, 116),
        warning: Color::Rgb(250, 189, 47),
    }
}

fn theme_catppuccin() -> Theme {
    Theme {
        bg: Color::Rgb(30, 30, 46),
        fg: Color::Rgb(205, 214, 244),
        accent: Color::Rgb(203, 166, 247),
        muted: Color::Rgb(108, 112, 134),
        warning: Color::Rgb(249, 226, 175),
    }
}
