use demo_support::{DemoConfig, DemoRenderer, sample_log_lines, Panel, theme_dracula};

fn main() {
    let config = DemoConfig {
        header_text: "titus",
        file_line: Some("File: /var/log/system.log"),
        footer_text: Some("<Theme: ^t> <Search: ^f> <Nav: ↑/↓> <Quit: q>"),
        log_title: "log view",
        log_lines: sample_log_lines(),
        left_panel: None,
        top_panel: None,
        overlay: Some(Panel { title: "Theme", lines: vec!["• Dracula (default)",
            "• Catppuccin Mocha",
            "• Solarized Dark",
            "• Gruvbox Dark",
            "[Apply] [Cancel]"] }),
        overlay_footer: None,
        theme: theme_dracula(),
    };

    let mut renderer = DemoRenderer::new(96, 28);
    renderer.render(&config);
    renderer.write_svg("../../theme-support/selector.svg");
}
