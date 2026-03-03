use demo_support::{DemoConfig, DemoRenderer, sample_log_lines, Panel, theme_dracula};

fn main() {
    let config = DemoConfig {
        header_text: "titus",
        file_line: Some("File: /var/log/system.log"),
        footer_text: Some("<Search: ^f> <Nav: ↑/↓> <Page: ^↑/^↓> <Quit: q> <Go: g>"),
        log_title: "log view",
        log_lines: sample_log_lines(),
        left_panel: None,
        top_panel: Some(Panel { title: "Jump to line", lines: vec!["[ 12054 ]",
            "Range: 1..46832"] }),
        overlay: None,
        overlay_footer: None,
        theme: theme_dracula(),
    };

    let mut renderer = DemoRenderer::new(96, 28);
    renderer.render(&config);
    renderer.write_svg("../../go-to-line/inline.svg");
}
