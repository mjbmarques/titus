use demo_support::{DemoConfig, DemoRenderer, sample_log_lines, Panel, theme_dracula};

fn main() {
    let config = DemoConfig {
        header_text: "titus",
        file_line: None,
        footer_text: Some("Files: system.log ▾"),
        log_title: "log view",
        log_lines: sample_log_lines(),
        left_panel: None,
        top_panel: Some(Panel { title: "Files", lines: vec!["system.log",
            "access.log",
            "error.log",
            "audit.log"] }),
        overlay: None,
        overlay_footer: None,
        theme: theme_dracula(),
    };

    let mut renderer = DemoRenderer::new(96, 28);
    renderer.render(&config);
    renderer.write_svg("../../multiple-files/dropdown.svg");
}
