use demo_support::{DemoConfig, DemoRenderer, sample_log_lines, Panel, theme_dracula};

fn main() {
    let config = DemoConfig {
        header_text: "titus",
        file_line: None,
        footer_text: Some("Find: error (Matches: 14) [Prev] [Next]"),
        log_title: "log view",
        log_lines: sample_log_lines(),
        left_panel: Some(Panel { title: "Matches", lines: vec!["12054 WARN Disk usage high",
            "12088 ERROR Database error",
            "12102 ERROR Connection reset",
            "12120 WARN Disk usage high"] }),
        top_panel: None,
        overlay: None,
        overlay_footer: None,
        theme: theme_dracula(),
    };

    let mut renderer = DemoRenderer::new(96, 28);
    renderer.render(&config);
    renderer.write_svg("../../log-search/sidebar.svg");
}
