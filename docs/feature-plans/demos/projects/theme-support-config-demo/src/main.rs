use demo_support::{DemoConfig, DemoRenderer, sample_log_lines, Panel, theme_catppuccin};

fn main() {
    let config = DemoConfig {
        header_text: "titus [catppuccin-mocha]",
        file_line: Some("File: /var/log/system.log"),
        footer_text: Some("<Theme: ^t> <Search: ^f> <Nav: ↑/↓> <Quit: q>"),
        log_title: "log view",
        log_lines: sample_log_lines(),
        left_panel: Some(Panel { title: "config/themes.yaml", lines: vec!["current: catppuccin-mocha",
            "",
            "palettes:",
            "  catppuccin-mocha:",
            "    background: #1E1E2E",
            "    foreground: #CDD6F4",
            "    accent: #CBA6F7",
            "    warning: #F9E2AF"] }),
        top_panel: None,
        overlay: None,
        overlay_footer: None,
        theme: theme_catppuccin(),
    };

    let mut renderer = DemoRenderer::new(96, 28);
    renderer.render(&config);
    renderer.write_svg("../../theme-support/config.svg");
}
