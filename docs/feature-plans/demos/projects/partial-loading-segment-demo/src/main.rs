use demo_support::{DemoConfig, DemoRenderer, sample_log_lines, theme_dracula};

fn main() {
    let config = DemoConfig {
        header_text: "titus",
        file_line: Some("File: /var/log/system.log (Segment 3/12: lines 10001..15000)"),
        footer_text: Some("Segment: [Prev] 3/12 [Next] Jump: g Search: ^f"),
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
    renderer.write_svg("../../partial-loading/segment.svg");
}
