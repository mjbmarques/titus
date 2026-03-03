use demo_support::{DemoConfig, DemoRenderer, sample_log_lines, theme_dracula};

fn main() {
    let config = DemoConfig {
        header_text: "titus",
        file_line: None,
        footer_text: Some("Go to line: 12054 (Enter to jump, Esc to cancel)"),
        log_title: "log view",
        log_lines: sample_log_lines(),
        left_panel: None,
        top_panel: None,
        overlay: None,
        overlay_footer: None,
        theme: theme_dracula(),
    };

    let mut renderer = DemoRenderer::new(96, 28);
    renderer.render(&config);
    renderer.write_svg("../../go-to-line/status.svg");
}
