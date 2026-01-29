use demo_support::{DemoConfig, DemoRenderer, sample_log_lines, Panel, theme_dracula};

fn main() {
    let config = DemoConfig {
        header_text: "titus",
        file_line: Some("File: /var/log/system.log"),
        footer_text: Some("<Search: ^f> <Nav: ↑/↓> <Page: ^↑/^↓> <Quit: q> <Import: ^o>"),
        log_title: "log view",
        log_lines: sample_log_lines(),
        left_panel: Some(Panel { title: "Files", lines: vec!["/var/log/",
            "└ system.log",
            "└ nginx/",
            "  └ access.log",
            "  └ error.log",
            "└ auth.log"] }),
        top_panel: None,
        overlay: None,
        overlay_footer: None,
        theme: theme_dracula(),
    };

    let mut renderer = DemoRenderer::new(96, 28);
    renderer.render(&config);
    renderer.write_svg("../../file-import/sidebar.svg");
}
